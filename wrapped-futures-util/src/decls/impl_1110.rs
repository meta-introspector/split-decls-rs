macro_rules! deps {
    () => {
        Ready!();
        Seek!();
        Cursor!();
    };
}

macro_rules! impl_1110 {
    () => {
        deps!();
        impl < T > AsyncSeek for Cursor < T > where T : AsRef < [u8] > + Unpin , { fn poll_seek (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < io :: Result < u64 > > { Poll :: Ready (io :: Seek :: seek (& mut self . inner , pos)) } }
    };
}

impl_1110!()