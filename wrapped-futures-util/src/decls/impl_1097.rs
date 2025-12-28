macro_rules! impl_1097 {
    () => {
        impl < R : AsyncRead , W : AsyncWrite + Unpin + ? Sized > Future for Copy < '_ , R , W > { type Output = io :: Result < u64 > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . inner . poll (cx) } }
    };
}

impl_1097!();