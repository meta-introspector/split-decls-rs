macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl < St , Fut , T , F > Future for TryFold < St , Fut , T , F > where St : Stream , F : FnMut (T , St :: Item) -> Fut , Fut : TryFuture < Ok = T > , { type Output = Result < T , Fut :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (fut) = this . future . as_mut () . as_pin_mut () { let res = ready ! (fut . try_poll (cx)) ; this . future . set (None) ; match res { Ok (a) => * this . accum = Some (a) , Err (e) => break Err (e) , } } else if this . accum . is_some () { let res = ready ! (this . stream . as_mut () . poll_next (cx)) ; let a = this . accum . take () . unwrap () ; match res { Some (item) => this . future . set (Some ((this . f) (a , item))) , None => break Ok (a) , } } else { panic ! ("Fold polled after completion") } }) } }
    };
}

impl_473!();