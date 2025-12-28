macro_rules! deps {
    () => {
        WriteHalf!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl < T : AsyncWrite + Unpin > AsyncWrite for WriteHalf < T > { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8]) -> Poll < Result < usize > > { let mut inner = self . 0 . lock () . unwrap () ; Pin :: new (& mut * inner) . poll_write (cx , buf) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { let mut inner = self . 0 . lock () . unwrap () ; Pin :: new (& mut * inner) . poll_flush (cx) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { let mut inner = self . 0 . lock () . unwrap () ; Pin :: new (& mut * inner) . poll_close (cx) } }
    };
}

impl_322!()