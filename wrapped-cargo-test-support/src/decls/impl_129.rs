macro_rules! deps {
    () => {
        HttpServerHandle!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl HttpServerHandle { pub fn index_url (& self) -> Url { Url :: parse (& format ! ("sparse+http://{}/index/" , self . addr)) . unwrap () } pub fn api_url (& self) -> Url { Url :: parse (& format ! ("http://{}/" , self . addr)) . unwrap () } pub fn dl_url (& self) -> Url { Url :: parse (& format ! ("http://{}/dl" , self . addr)) . unwrap () } fn stop (& self) { if let Ok (mut stream) = TcpStream :: connect (self . addr) { let _ = stream . write_all (b"stop") ; let _ = stream . flush () ; } } }
    };
}

impl_129!()