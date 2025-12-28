macro_rules! CurlSubtransport {
    () => {
        struct CurlSubtransport { handle : Arc < Mutex < Easy > > , service : & 'static str , url_path : & 'static str , base_url : Arc < Mutex < String > > , method : & 'static str , reader : Option < Cursor < Vec < u8 > > > , sent_request : bool , }
    };
}

CurlSubtransport!()