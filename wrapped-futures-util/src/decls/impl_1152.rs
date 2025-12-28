macro_rules! deps {
    () => {
        ReadVectored!();
    };
}

macro_rules! impl_1152 {
    () => {
        deps!();
        impl < R : AsyncRead + ? Sized + Unpin > Future for ReadVectored < '_ , '_ , R > { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; Pin :: new (& mut this . reader) . poll_read_vectored (cx , this . bufs) } }
    };
}

impl_1152!()