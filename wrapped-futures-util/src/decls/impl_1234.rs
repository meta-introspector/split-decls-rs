macro_rules! deps {
    () => {
        WriteVectored!();
    };
}

macro_rules! impl_1234 {
    () => {
        deps!();
        impl < W : AsyncWrite + ? Sized + Unpin > Future for WriteVectored < '_ , '_ , W > { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; Pin :: new (& mut this . writer) . poll_write_vectored (cx , this . bufs) } }
    };
}

impl_1234!();