macro_rules! deps {
    () => {
        Flush!();
    };
}

macro_rules! impl_1133 {
    () => {
        deps!();
        impl < W > Future for Flush < '_ , W > where W : AsyncWrite + ? Sized + Unpin , { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut * self . writer) . poll_flush (cx) } }
    };
}

impl_1133!()