macro_rules! deps {
    () => {
        Ready!();
        Empty!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < Fut > Stream for Flatten < Fut , Fut :: Output > where Fut : Future , Fut :: Output : Stream , { type Item = < Fut :: Output as Stream > :: Item ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (loop { match self . as_mut () . project () { FlattenProj :: First { f } => { let f = ready ! (f . poll (cx)) ; self . set (Self :: Second { f }) ; } FlattenProj :: Second { f } => { let output = ready ! (f . poll_next (cx)) ; if output . is_none () { self . set (Self :: Empty) ; } break output ; } FlattenProj :: Empty => break None , } }) } }
    };
}

impl_43!();