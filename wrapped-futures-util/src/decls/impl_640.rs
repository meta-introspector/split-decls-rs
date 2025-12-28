macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_640 {
    () => {
        deps!();
        impl < St > Stream for TryFlatten < St > where St : TryStream , St :: Ok : TryStream , < St :: Ok as TryStream > :: Error : From < St :: Error > , { type Item = Result < < St :: Ok as TryStream > :: Ok , < St :: Ok as TryStream > :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (s) = this . next . as_mut () . as_pin_mut () { if let Some (item) = ready ! (s . try_poll_next (cx) ?) { break Some (Ok (item)) ; } else { this . next . set (None) ; } } else if let Some (s) = ready ! (this . stream . as_mut () . try_poll_next (cx) ?) { this . next . set (Some (s)) ; } else { break None ; } }) } }
    };
}

impl_640!()