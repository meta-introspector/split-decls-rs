macro_rules! deps {
    () => {
        Sink!();
        Ready!();
    };
}

macro_rules! impl_973 {
    () => {
        deps!();
        impl < Si , Item , U , Fut , F , E > Sink < U > for With < Si , Item , U , Fut , F > where Si : Sink < Item > , F : FnMut (U) -> Fut , Fut : Future < Output = Result < Item , E > > , E : From < Si :: Error > , { type Error = E ; fn poll_ready (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { ready ! (self . as_mut () . poll (cx)) ? ; ready ! (self . project () . sink . poll_ready (cx) ?) ; Poll :: Ready (Ok (())) } fn start_send (self : Pin < & mut Self > , item : U) -> Result < () , Self :: Error > { let mut this = self . project () ; assert ! (this . state . is_none ()) ; this . state . set (Some ((this . f) (item))) ; Ok (()) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { ready ! (self . as_mut () . poll (cx)) ? ; ready ! (self . project () . sink . poll_flush (cx) ?) ; Poll :: Ready (Ok (())) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { ready ! (self . as_mut () . poll (cx)) ? ; ready ! (self . project () . sink . poll_close (cx) ?) ; Poll :: Ready (Ok (())) } }
    };
}

impl_973!()