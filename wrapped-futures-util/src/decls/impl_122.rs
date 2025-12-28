macro_rules! deps {
    () => {
        Sink!();
        Empty!();
        Ready!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < Fut , Item > Sink < Item > for TryFlatten < Fut , Fut :: Ok > where Fut : TryFuture , Fut :: Ok : Sink < Item , Error = Fut :: Error > , { type Error = Fut :: Error ; fn poll_ready (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (loop { match self . as_mut () . project () { TryFlattenProj :: First { f } => match ready ! (f . try_poll (cx)) { Ok (f) => self . set (Self :: Second { f }) , Err (e) => { self . set (Self :: Empty) ; break Err (e) ; } } , TryFlattenProj :: Second { f } => { break ready ! (f . poll_ready (cx)) ; } TryFlattenProj :: Empty => panic ! ("poll_ready called after eof") , } }) } fn start_send (self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { match self . project () { TryFlattenProj :: First { .. } => panic ! ("poll_ready not called first") , TryFlattenProj :: Second { f } => f . start_send (item) , TryFlattenProj :: Empty => panic ! ("start_send called after eof") , } } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match self . project () { TryFlattenProj :: First { .. } => Poll :: Ready (Ok (())) , TryFlattenProj :: Second { f } => f . poll_flush (cx) , TryFlattenProj :: Empty => panic ! ("poll_flush called after eof") , } } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { let res = match self . as_mut () . project () { TryFlattenProj :: Second { f } => f . poll_close (cx) , _ => Poll :: Ready (Ok (())) , } ; if res . is_ready () { self . set (Self :: Empty) ; } res } }
    };
}

impl_122!();