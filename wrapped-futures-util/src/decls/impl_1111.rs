macro_rules! deps {
    () => {
        Ready!();
        Read!();
        Cursor!();
    };
}

macro_rules! impl_1111 {
    () => {
        deps!();
        impl < T : AsRef < [u8] > + Unpin > AsyncRead for Cursor < T > { fn poll_read (mut self : Pin < & mut Self > , _cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (io :: Read :: read (& mut self . inner , buf)) } fn poll_read_vectored (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (io :: Read :: read_vectored (& mut self . inner , bufs)) } }
    };
}

impl_1111!()