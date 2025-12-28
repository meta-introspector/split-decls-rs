macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < T > AsyncRead for Cursor < T > where T : AsRef < [u8] > + Unpin , { fn poll_read (mut self : Pin < & mut Self > , _cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize > > { Poll :: Ready (std :: io :: Read :: read (& mut self . inner , buf)) } fn poll_read_vectored (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < Result < usize > > { Poll :: Ready (std :: io :: Read :: read_vectored (& mut self . inner , bufs)) } }
    };
}

impl_230!()