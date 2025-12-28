macro_rules! deps {
    () => {
        WriteVectoredFuture!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl < W : AsyncWrite + Unpin + ? Sized > Future for WriteVectoredFuture < '_ , W > { type Output = Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let bufs = self . bufs ; Pin :: new (& mut * self . writer) . poll_write_vectored (cx , bufs) } }
    };
}

impl_306!();