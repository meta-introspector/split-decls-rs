macro_rules! deps {
    () => {
        FlushFuture!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl < W : AsyncWrite + Unpin + ? Sized > Future for FlushFuture < '_ , W > { type Output = Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut * self . writer) . poll_flush (cx) } }
    };
}

impl_312!();