macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl < St , Fut , F > Future for All < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = bool > , { type Output = bool ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < bool > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (fut) = this . future . as_mut () . as_pin_mut () { let res = ready ! (fut . poll (cx)) ; this . future . set (None) ; if ! res { * this . done = true ; break false ; } } else if ! * this . done { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (item) => { this . future . set (Some ((this . f) (item))) ; } None => { * this . done = true ; break true ; } } } else { panic ! ("All polled after completion") } }) } }
    };
}

impl_355!();