macro_rules! deps {
    () => {
        CloseFuture!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < W : AsyncWrite + Unpin + ? Sized > Future for CloseFuture < '_ , W > { type Output = Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut * self . writer) . poll_close (cx) } }
    };
}

impl_315!()