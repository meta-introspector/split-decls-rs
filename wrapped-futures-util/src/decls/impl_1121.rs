macro_rules! deps {
    () => {
        Empty!();
        Ready!();
    };
}

macro_rules! impl_1121 {
    () => {
        deps!();
        impl AsyncRead for Empty { # [inline] fn poll_read (self : Pin < & mut Self > , _ : & mut Context < '_ > , _ : & mut [u8] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Ok (0)) } }
    };
}

impl_1121!()