macro_rules! deps {
    () => {
        TryFoldFuture!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < T , E , S , F , B > Future for TryFoldFuture < '_ , S , F , B > where S : Stream < Item = Result < T , E > > + Unpin , F : FnMut (B , T) -> Result < B , E > , { type Output = Result < B , E > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . stream . poll_next (cx)) { Some (Err (e)) => return Poll :: Ready (Err (e)) , Some (Ok (t)) => { let old = self . acc . take () . unwrap () ; let new = (& mut self . f) (old , t) ; match new { Ok (t) => self . acc = Some (t) , Err (e) => return Poll :: Ready (Err (e)) , } } None => return Poll :: Ready (Ok (self . acc . take () . unwrap ())) , } } } }
    };
}

impl_108!();