macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_1218 {
    () => {
        deps!();
        impl < R : AsyncRead > AsyncRead for Take < R > { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize , io :: Error > > { let this = self . project () ; if * this . limit == 0 { return Poll :: Ready (Ok (0)) ; } let max = cmp :: min (buf . len () as u64 , * this . limit) as usize ; let n = ready ! (this . inner . poll_read (cx , & mut buf [.. max])) ? ; * this . limit -= n as u64 ; Poll :: Ready (Ok (n)) } }
    };
}

impl_1218!();