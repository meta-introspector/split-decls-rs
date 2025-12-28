macro_rules! deps {
    () => {
        InterleavePending!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < R : AsyncBufRead > AsyncBufRead for InterleavePending < R > { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { self . poll_with (cx , R :: poll_fill_buf) } fn consume (self : Pin < & mut Self > , amount : usize) { self . project () . inner . consume (amount) } }
    };
}

impl_94!();