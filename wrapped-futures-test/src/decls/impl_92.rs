macro_rules! deps {
    () => {
        InterleavePending!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < W : AsyncWrite > AsyncWrite for InterleavePending < W > { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { self . poll_with (cx , | w , cx | w . poll_write (cx , buf)) } fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { self . poll_with (cx , | w , cx | w . poll_write_vectored (cx , bufs)) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . poll_with (cx , W :: poll_flush) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . poll_with (cx , W :: poll_close) } }
    };
}

impl_92!()