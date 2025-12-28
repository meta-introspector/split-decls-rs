macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        impl < St , Fut , F > Future for ForEach < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = () > , { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < () > { let mut this = self . project () ; loop { if let Some (fut) = this . future . as_mut () . as_pin_mut () { ready ! (fut . poll (cx)) ; this . future . set (None) ; } else if let Some (item) = ready ! (this . stream . as_mut () . poll_next (cx)) { this . future . set (Some ((this . f) (item))) ; } else { break ; } } Poll :: Ready (()) } }
    };
}

impl_367!();