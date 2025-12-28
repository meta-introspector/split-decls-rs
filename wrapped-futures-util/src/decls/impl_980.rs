macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_980 {
    () => {
        deps!();
        impl < Si , Item , U , St , F > Sink < U > for WithFlatMap < Si , Item , U , St , F > where Si : Sink < Item > , F : FnMut (U) -> St , St : Stream < Item = Result < Item , Si :: Error > > , { type Error = Si :: Error ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . try_empty_stream (cx) } fn start_send (self : Pin < & mut Self > , item : U) -> Result < () , Self :: Error > { let mut this = self . project () ; assert ! (this . stream . is_none ()) ; this . stream . set (Some ((this . f) (item))) ; Ok (()) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { ready ! (self . as_mut () . try_empty_stream (cx) ?) ; self . project () . sink . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { ready ! (self . as_mut () . try_empty_stream (cx) ?) ; self . project () . sink . poll_close (cx) } }
    };
}

impl_980!();