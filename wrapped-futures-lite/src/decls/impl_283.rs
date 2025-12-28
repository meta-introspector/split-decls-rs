macro_rules! impl_283 {
    () => {
        impl < R : AsyncRead > AsyncRead for Take < R > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize > > { let this = self . project () ; take_read_internal (this . inner , cx , buf , this . limit) } }
    };
}

impl_283!();