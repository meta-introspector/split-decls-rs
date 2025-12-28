macro_rules! deps {
    () => {
        Limited!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < R : AsyncRead > AsyncRead for Limited < R > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { let this = self . project () ; let limit = cmp :: min (* this . limit , buf . len ()) ; this . io . poll_read (cx , & mut buf [.. limit]) } }
    };
}

impl_60!();