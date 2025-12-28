macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_769 {
    () => {
        deps!();
        impl < Fut : Future > Stream for Once < Fut > { type Item = Fut :: Output ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; let v = match this . future . as_mut () . as_pin_mut () { Some (fut) => ready ! (fut . poll (cx)) , None => return Poll :: Ready (None) , } ; this . future . set (None) ; Poll :: Ready (Some (v)) } fn size_hint (& self) -> (usize , Option < usize >) { if self . future . is_some () { (1 , Some (1)) } else { (0 , Some (0)) } } }
    };
}

impl_769!();