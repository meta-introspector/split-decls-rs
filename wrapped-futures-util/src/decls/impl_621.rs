macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_621 {
    () => {
        deps!();
        impl < St , Fut , F > Stream for TryFilter < St , Fut , F > where St : TryStream , Fut : Future < Output = bool > , F : FnMut (& St :: Ok) -> Fut , { type Item = Result < St :: Ok , St :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (fut) = this . pending_fut . as_mut () . as_pin_mut () { let res = ready ! (fut . poll (cx)) ; this . pending_fut . set (None) ; if res { break this . pending_item . take () . map (Ok) ; } * this . pending_item = None ; } else if let Some (item) = ready ! (this . stream . as_mut () . try_poll_next (cx) ?) { this . pending_fut . set (Some ((this . f) (& item))) ; * this . pending_item = Some (item) ; } else { break None ; } }) } fn size_hint (& self) -> (usize , Option < usize >) { let pending_len = usize :: from (self . pending_fut . is_some ()) ; let (_ , upper) = self . stream . size_hint () ; let upper = match upper { Some (x) => x . checked_add (pending_len) , None => None , } ; (0 , upper) } }
    };
}

impl_621!();