macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! impl_1229 {
    () => {
        deps!();
        impl < W : AsyncWrite + ? Sized + Unpin > Future for Write < '_ , W > { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; Pin :: new (& mut this . writer) . poll_write (cx , this . buf) } }
    };
}

impl_1229!()