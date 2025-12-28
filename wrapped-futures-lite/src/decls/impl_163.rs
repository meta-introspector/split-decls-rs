macro_rules! deps {
    () => {
        FindFuture!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < S , P > Future for FindFuture < '_ , S , P > where S : Stream + Unpin + ? Sized , P : FnMut (& S :: Item) -> bool , { type Output = Option < S :: Item > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . stream . poll_next (cx)) { Some (v) if (& mut self . predicate) (& v) => return Poll :: Ready (Some (v)) , Some (_) => { } None => return Poll :: Ready (None) , } } } }
    };
}

impl_163!();