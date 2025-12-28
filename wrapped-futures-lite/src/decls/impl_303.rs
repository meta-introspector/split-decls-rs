macro_rules! deps {
    () => {
        WriteFuture!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl < W : AsyncWrite + Unpin + ? Sized > Future for WriteFuture < '_ , W > { type Output = Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let buf = self . buf ; Pin :: new (& mut * self . writer) . poll_write (cx , buf) } }
    };
}

impl_303!()