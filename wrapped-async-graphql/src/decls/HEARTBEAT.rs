macro_rules! HEARTBEAT {
    () => {
        static HEARTBEAT : Bytes = Bytes :: from_static (b"{}\r\n") ;
    };
}

HEARTBEAT!()