macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_730 {
    () => {
        deps!();
        impl < St , Fut , F > Future for TryAll < St , Fut , F > where St : TryStream , F : FnMut (St :: Ok) -> Fut , Fut : Future < Output = bool > , { type Output = Result < bool , St :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < bool , St :: Error > > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (fut) = this . future . as_mut () . as_pin_mut () { let acc = ready ! (fut . poll (cx)) ; this . future . set (None) ; if ! acc { * this . done = true ; break Ok (false) ; } } else if ! * this . done { match ready ! (this . stream . as_mut () . try_poll_next (cx)) { Some (Ok (item)) => { this . future . set (Some ((this . f) (item))) ; } Some (Err (err)) => { * this . done = true ; break Err (err) ; } None => { * this . done = true ; break Ok (true) ; } } } else { panic ! ("TryAll polled after completion") } }) } }
    };
}

impl_730!()