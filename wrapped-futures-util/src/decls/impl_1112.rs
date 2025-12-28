macro_rules! deps {
    () => {
        Ready!();
        Cursor!();
    };
}

macro_rules! impl_1112 {
    () => {
        deps!();
        impl < T > AsyncBufRead for Cursor < T > where T : AsRef < [u8] > + Unpin , { fn poll_fill_buf (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { Poll :: Ready (io :: BufRead :: fill_buf (& mut self . get_mut () . inner)) } fn consume (mut self : Pin < & mut Self > , amt : usize) { io :: BufRead :: consume (& mut self . inner , amt) } }
    };
}

impl_1112!()