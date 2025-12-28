macro_rules! deps {
    () => {
        ReadToStringFuture!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < R : AsyncRead + Unpin + ? Sized > Future for ReadToStringFuture < '_ , R > { type Output = Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , buf , bytes , start_len , } = & mut * self ; let reader = Pin :: new (reader) ; let ret = ready ! (read_to_end_internal (reader , cx , bytes , * start_len)) ; match String :: from_utf8 (mem :: take (bytes)) { Ok (s) => { debug_assert ! (buf . is_empty ()) ; * * buf = s ; Poll :: Ready (ret) } Err (_) => Poll :: Ready (ret . and_then (| _ | { Err (Error :: new (ErrorKind :: InvalidData , "stream did not contain valid UTF-8" ,)) })) , } } }
    };
}

impl_276!()