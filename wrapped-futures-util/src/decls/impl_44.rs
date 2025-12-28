macro_rules! deps {
    () => {
        Empty!();
        Ready!();
        Sink!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < Fut , Item > Sink < Item > for Flatten < Fut , Fut :: Output > where Fut : Future , Fut :: Output : Sink < Item > , { type Error = < Fut :: Output as Sink < Item > > :: Error ; fn poll_ready (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (loop { match self . as_mut () . project () { FlattenProj :: First { f } => { let f = ready ! (f . poll (cx)) ; self . set (Self :: Second { f }) ; } FlattenProj :: Second { f } => { break ready ! (f . poll_ready (cx)) ; } FlattenProj :: Empty => panic ! ("poll_ready called after eof") , } }) } fn start_send (self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { match self . project () { FlattenProj :: First { .. } => panic ! ("poll_ready not called first") , FlattenProj :: Second { f } => f . start_send (item) , FlattenProj :: Empty => panic ! ("start_send called after eof") , } } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match self . project () { FlattenProj :: First { .. } => Poll :: Ready (Ok (())) , FlattenProj :: Second { f } => f . poll_flush (cx) , FlattenProj :: Empty => panic ! ("poll_flush called after eof") , } } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { let res = match self . as_mut () . project () { FlattenProj :: Second { f } => f . poll_close (cx) , _ => Poll :: Ready (Ok (())) , } ; if res . is_ready () { self . set (Self :: Empty) ; } res } }
    };
}

impl_44!();