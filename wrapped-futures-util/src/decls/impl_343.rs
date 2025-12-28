macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl < St , Fut , T , F > Future for Fold < St , Fut , T , F > where St : Stream , F : FnMut (T , St :: Item) -> Fut , Fut : Future < Output = T > , { type Output = T ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (fut) = this . future . as_mut () . as_pin_mut () { * this . accum = Some (ready ! (fut . poll (cx))) ; this . future . set (None) ; } else if this . accum . is_some () { let res = ready ! (this . stream . as_mut () . poll_next (cx)) ; let a = this . accum . take () . unwrap () ; if let Some (item) = res { this . future . set (Some ((this . f) (a , item))) ; } else { break a ; } } else { panic ! ("Fold polled after completion") } }) } }
    };
}

impl_343!()