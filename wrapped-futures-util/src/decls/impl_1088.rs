macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_1088 {
    () => {
        deps!();
        impl < T , U > AsyncBufRead for Chain < T , U > where T : AsyncBufRead , U : AsyncBufRead , { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { let this = self . project () ; if ! * this . done_first { match ready ! (this . first . poll_fill_buf (cx) ?) { [] => * this . done_first = true , buf => return Poll :: Ready (Ok (buf)) , } } this . second . poll_fill_buf (cx) } fn consume (self : Pin < & mut Self > , amt : usize) { let this = self . project () ; if ! * this . done_first { this . first . consume (amt) } else { this . second . consume (amt) } } }
    };
}

impl_1088!();