macro_rules! deps {
    () => {
        ReadFuture!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl < R : AsyncRead + Unpin + ? Sized > Future for ReadFuture < '_ , R > { type Output = Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , buf } = & mut * self ; Pin :: new (reader) . poll_read (cx , buf) } }
    };
}

impl_267!();