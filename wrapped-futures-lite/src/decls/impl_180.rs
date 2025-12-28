macro_rules! deps {
    () => {
        TryForEachFuture!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < S , F , E > Future for TryForEachFuture < '_ , S , F > where S : Stream + Unpin + ? Sized , F : FnMut (S :: Item) -> Result < () , E > , { type Output = Result < () , E > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . stream . poll_next (cx)) { None => return Poll :: Ready (Ok (())) , Some (v) => (& mut self . f) (v) ? , } } } }
    };
}

impl_180!();