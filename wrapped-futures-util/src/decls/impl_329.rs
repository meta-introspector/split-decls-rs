macro_rules! deps {
    () => {
        FnMut1!();
        Ready!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl < St , Fut , F , T > Stream for FilterMap < St , Fut , F > where St : Stream , F : FnMut1 < St :: Item , Output = Fut > , Fut : Future < Output = Option < T > > , { type Item = T ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < T > > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (p) = this . pending . as_mut () . as_pin_mut () { let item = ready ! (p . poll (cx)) ; this . pending . set (None) ; if item . is_some () { break item ; } } else if let Some (item) = ready ! (this . stream . as_mut () . poll_next (cx)) { this . pending . set (Some (this . f . call_mut (item))) ; } else { break None ; } }) } fn size_hint (& self) -> (usize , Option < usize >) { let pending_len = usize :: from (self . pending . is_some ()) ; let (_ , upper) = self . stream . size_hint () ; let upper = match upper { Some (x) => x . checked_add (pending_len) , None => None , } ; (0 , upper) } }
    };
}

impl_329!()