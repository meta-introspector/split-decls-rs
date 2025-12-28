macro_rules! deps {
    () => {
        ReadVectoredFuture!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < R : AsyncRead + Unpin + ? Sized > Future for ReadVectoredFuture < '_ , R > { type Output = Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , bufs } = & mut * self ; Pin :: new (reader) . poll_read_vectored (cx , bufs) } }
    };
}

impl_270!();