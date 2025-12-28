macro_rules! deps {
    () => {
        NextFuture!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < S : Stream + Unpin + ? Sized > Future for NextFuture < '_ , S > { type Output = Option < S :: Item > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . stream . poll_next (cx) } }
    };
}

impl_92!();