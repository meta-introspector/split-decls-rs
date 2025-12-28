macro_rules! deps {
    () => {
        Repeat!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl AsyncRead for Repeat { # [inline] fn poll_read (self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & mut [u8]) -> Poll < Result < usize > > { for b in & mut * buf { * b = self . byte ; } Poll :: Ready (Ok (buf . len ())) } }
    };
}

impl_242!()