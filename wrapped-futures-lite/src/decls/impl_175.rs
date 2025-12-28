macro_rules! deps {
    () => {
        AnyFuture!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < S , P > Future for AnyFuture < '_ , S , P > where S : Stream + Unpin + ? Sized , P : FnMut (S :: Item) -> bool , { type Output = bool ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . stream . poll_next (cx)) { Some (v) => { if (& mut self . predicate) (v) { return Poll :: Ready (true) ; } } None => return Poll :: Ready (false) , } } } }
    };
}

impl_175!();