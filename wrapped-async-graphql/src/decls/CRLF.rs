macro_rules! CRLF {
    () => {
        static CRLF : Bytes = Bytes :: from_static (b"\r\n") ;
    };
}

CRLF!()