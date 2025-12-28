macro_rules! deps {
    () => {
        ReadHalf!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl < T : AsyncRead + Unpin > AsyncRead for ReadHalf < T > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize > > { let mut inner = self . 0 . lock () . unwrap () ; Pin :: new (& mut * inner) . poll_read (cx , buf) } fn poll_read_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < Result < usize > > { let mut inner = self . 0 . lock () . unwrap () ; Pin :: new (& mut * inner) . poll_read_vectored (cx , bufs) } }
    };
}

impl_321!()