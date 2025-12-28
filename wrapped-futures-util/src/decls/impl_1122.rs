macro_rules! deps {
    () => {
        Empty!();
        Ready!();
    };
}

macro_rules! impl_1122 {
    () => {
        deps!();
        impl AsyncBufRead for Empty { # [inline] fn poll_fill_buf (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { Poll :: Ready (Ok (& [])) } # [inline] fn consume (self : Pin < & mut Self > , _ : usize) { } }
    };
}

impl_1122!()