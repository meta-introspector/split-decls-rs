macro_rules! deps {
    () => {
        Read!();
        Result!();
        Error!();
        Pending!();
        ReadBufCursor!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl Read for hyper_io { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , mut buf : crate :: rt :: ReadBufCursor < '_ > ,) -> Poll < std :: io :: Result < () > > { let buf_ptr = unsafe { buf . as_mut () } . as_mut_ptr () as * mut u8 ; let buf_len = buf . remaining () ; match (self . read) (self . userdata , hyper_context :: wrap (cx) , buf_ptr , buf_len) { HYPER_IO_PENDING => Poll :: Pending , HYPER_IO_ERROR => Poll :: Ready (Err (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "io error" ,))) , ok => { unsafe { buf . advance (ok) } ; Poll :: Ready (Ok (())) } } } }
    };
}

impl_325!()