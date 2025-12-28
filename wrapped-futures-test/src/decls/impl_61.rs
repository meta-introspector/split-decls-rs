macro_rules! deps {
    () => {
        Limited!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < R : AsyncBufRead > AsyncBufRead for Limited < R > { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { self . project () . io . poll_fill_buf (cx) } fn consume (self : Pin < & mut Self > , amount : usize) { self . project () . io . consume (amount) } }
    };
}

impl_61!()