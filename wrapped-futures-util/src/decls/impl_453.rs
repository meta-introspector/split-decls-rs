macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_453 {
    () => {
        deps!();
        impl < St , Fut > Stream for TakeUntil < St , Fut > where St : Stream , Fut : Future , { type Item = St :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < St :: Item > > { let mut this = self . project () ; if let Some (f) = this . fut . as_mut () . as_pin_mut () { if let Poll :: Ready (result) = f . poll (cx) { this . fut . set (None) ; * this . fut_result = Some (result) ; } } if ! * this . free && this . fut . is_none () { Poll :: Ready (None) } else { let item = ready ! (this . stream . poll_next (cx)) ; if item . is_none () { this . fut . set (None) ; } Poll :: Ready (item) } } fn size_hint (& self) -> (usize , Option < usize >) { if self . is_stopped () { return (0 , Some (0)) ; } self . stream . size_hint () } }
    };
}

impl_453!()