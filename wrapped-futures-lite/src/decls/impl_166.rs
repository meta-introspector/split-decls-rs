macro_rules! deps {
    () => {
        FindMapFuture!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < S , B , F > Future for FindMapFuture < '_ , S , F > where S : Stream + Unpin + ? Sized , F : FnMut (S :: Item) -> Option < B > , { type Output = Option < B > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . stream . poll_next (cx)) { Some (v) => { if let Some (v) = (& mut self . f) (v) { return Poll :: Ready (Some (v)) ; } } None => return Poll :: Ready (None) , } } } }
    };
}

impl_166!()