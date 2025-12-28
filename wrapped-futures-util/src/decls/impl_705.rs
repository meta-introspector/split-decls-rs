macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_705 {
    () => {
        deps!();
        impl < St , Fut , F > Stream for TryTakeWhile < St , Fut , F > where St : TryStream , F : FnMut (& St :: Ok) -> Fut , Fut : TryFuture < Ok = bool , Error = St :: Error > , { type Item = Result < St :: Ok , St :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; if * this . done_taking { return Poll :: Ready (None) ; } Poll :: Ready (loop { if let Some (fut) = this . pending_fut . as_mut () . as_pin_mut () { let res = ready ! (fut . try_poll (cx)) ; this . pending_fut . set (None) ; let take = res ? ; let item = this . pending_item . take () ; if take { break item . map (Ok) ; } else { * this . done_taking = true ; break None ; } } else if let Some (item) = ready ! (this . stream . as_mut () . try_poll_next (cx) ?) { this . pending_fut . set (Some ((this . f) (& item))) ; * this . pending_item = Some (item) ; } else { break None ; } }) } fn size_hint (& self) -> (usize , Option < usize >) { if self . done_taking { return (0 , Some (0)) ; } let pending_len = usize :: from (self . pending_item . is_some ()) ; let (_ , upper) = self . stream . size_hint () ; let upper = match upper { Some (x) => x . checked_add (pending_len) , None => None , } ; (0 , upper) } }
    };
}

impl_705!();