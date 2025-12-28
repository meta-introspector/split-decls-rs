macro_rules! deps {
    () => {
        RequestWriter!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl futures_io :: AsyncWrite for RequestWriter < '_ > { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8]) -> Poll < io :: Result < usize > > { self . project () . writer . poll_write (cx , buf) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . project () . writer . poll_flush (cx) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . project () . writer . poll_close (cx) } }
    };
}

impl_14!()