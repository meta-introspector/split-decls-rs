macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl < T > AsyncSeek for Cursor < T > where T : AsRef < [u8] > + Unpin , { fn poll_seek (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < Result < u64 > > { Poll :: Ready (std :: io :: Seek :: seek (& mut self . inner , pos)) } }
    };
}

impl_229!();