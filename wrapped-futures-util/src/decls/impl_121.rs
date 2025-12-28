macro_rules! deps {
    () => {
        Ready!();
        Empty!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < Fut > Stream for TryFlatten < Fut , Fut :: Ok > where Fut : TryFuture , Fut :: Ok : TryStream < Error = Fut :: Error > , { type Item = Result < < Fut :: Ok as TryStream > :: Ok , Fut :: Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (loop { match self . as_mut () . project () { TryFlattenProj :: First { f } => match ready ! (f . try_poll (cx)) { Ok (f) => self . set (Self :: Second { f }) , Err (e) => { self . set (Self :: Empty) ; break Some (Err (e)) ; } } , TryFlattenProj :: Second { f } => { let output = ready ! (f . try_poll_next (cx)) ; if output . is_none () { self . set (Self :: Empty) ; } break output ; } TryFlattenProj :: Empty => break None , } }) } }
    };
}

impl_121!();