macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_589 {
    () => {
        deps!();
        impl < St , Fut , F > Stream for AndThen < St , Fut , F > where St : TryStream , F : FnMut (St :: Ok) -> Fut , Fut : TryFuture < Error = St :: Error > , { type Item = Result < Fut :: Ok , St :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (fut) = this . future . as_mut () . as_pin_mut () { let item = ready ! (fut . try_poll (cx)) ; this . future . set (None) ; break Some (item) ; } else if let Some (item) = ready ! (this . stream . as_mut () . try_poll_next (cx) ?) { this . future . set (Some ((this . f) (item))) ; } else { break None ; } }) } fn size_hint (& self) -> (usize , Option < usize >) { let future_len = usize :: from (self . future . is_some ()) ; let (lower , upper) = self . stream . size_hint () ; let lower = lower . saturating_add (future_len) ; let upper = match upper { Some (x) => x . checked_add (future_len) , None => None , } ; (lower , upper) } }
    };
}

impl_589!()