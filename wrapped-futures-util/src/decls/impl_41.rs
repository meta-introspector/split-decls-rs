macro_rules! deps {
    () => {
        Empty!();
        Ready!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < Fut > Future for Flatten < Fut , Fut :: Output > where Fut : Future , Fut :: Output : Future , { type Output = < Fut :: Output as Future > :: Output ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Poll :: Ready (loop { match self . as_mut () . project () { FlattenProj :: First { f } => { let f = ready ! (f . poll (cx)) ; self . set (Self :: Second { f }) ; } FlattenProj :: Second { f } => { let output = ready ! (f . poll (cx)) ; self . set (Self :: Empty) ; break output ; } FlattenProj :: Empty => panic ! ("Flatten polled after completion") , } }) } }
    };
}

impl_41!()