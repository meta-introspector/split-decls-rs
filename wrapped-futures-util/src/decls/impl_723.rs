macro_rules! impl_723 {
    () => {
        impl < St > AsyncWrite for IntoAsyncRead < St > where St : TryStream < Error = Error > + AsyncWrite , St :: Ok : AsRef < [u8] > , { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8]) -> Poll < Result < usize > > { let this = self . project () ; this . stream . poll_write (cx , buf) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { let this = self . project () ; this . stream . poll_flush (cx) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { let this = self . project () ; this . stream . poll_close (cx) } }
    };
}

impl_723!()