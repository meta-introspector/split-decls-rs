macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl AsyncRead for Empty { # [inline] fn poll_read (self : Pin < & mut Self > , _ : & mut Context < '_ > , _ : & mut [u8]) -> Poll < Result < usize > > { Poll :: Ready (Ok (0)) } }
    };
}

impl_238!()