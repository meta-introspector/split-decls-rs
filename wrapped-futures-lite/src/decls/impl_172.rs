macro_rules! deps {
    () => {
        AllFuture!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < S , P > Future for AllFuture < '_ , S , P > where S : Stream + Unpin + ? Sized , P : FnMut (S :: Item) -> bool , { type Output = bool ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . stream . poll_next (cx)) { Some (v) => { if ! (& mut self . predicate) (v) { return Poll :: Ready (false) ; } } None => return Poll :: Ready (true) , } } } }
    };
}

impl_172!();