macro_rules! HttpServerHandle {
    () => {
        pub struct HttpServerHandle { addr : SocketAddr , handle : Option < JoinHandle < () > > , }
    };
}

HttpServerHandle!()