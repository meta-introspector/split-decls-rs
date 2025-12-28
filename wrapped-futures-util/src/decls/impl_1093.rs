macro_rules! deps {
    () => {
        Close!();
    };
}

macro_rules! impl_1093 {
    () => {
        deps!();
        impl < W : AsyncWrite + ? Sized + Unpin > Future for Close < '_ , W > { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut * self . writer) . poll_close (cx) } }
    };
}

impl_1093!();