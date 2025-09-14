fn main() {
    let string = String::from();
    
    /*
    let server = Server::new("127.0.0.1:8080");
    server.run();
    */
}

struct Server {
    addr: String,
}

impl Server {
    fn new( addr: String ) -> Self {
        Self { addr }
    }
    fn run( self ) {

    }
}