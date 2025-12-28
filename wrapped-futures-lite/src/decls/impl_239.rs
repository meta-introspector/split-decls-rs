macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl AsyncBufRead for Empty { # [inline] fn poll_fill_buf < 'a > (self : Pin < & 'a mut Self > , _ : & mut Context < '_ >) -> Poll < Result < & 'a [u8] > > { Poll :: Ready (Ok (& [])) } # [inline] fn consume (self : Pin < & mut Self > , _ : usize) { } }
    };
}

impl_239!()