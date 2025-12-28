macro_rules! deps {
    () => {
        Pending!();
        Ready!();
        Sink!();
    };
}

macro_rules! impl_928 {
    () => {
        deps!();
        impl < Si1 , Si2 , Item > Sink < Item > for Fanout < Si1 , Si2 > where Si1 : Sink < Item > , Item : Clone , Si2 : Sink < Item , Error = Si1 :: Error > , { type Error = Si1 :: Error ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { let this = self . project () ; let sink1_ready = this . sink1 . poll_ready (cx) ? . is_ready () ; let sink2_ready = this . sink2 . poll_ready (cx) ? . is_ready () ; let ready = sink1_ready && sink2_ready ; if ready { Poll :: Ready (Ok (())) } else { Poll :: Pending } } fn start_send (self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { let this = self . project () ; this . sink1 . start_send (item . clone ()) ? ; this . sink2 . start_send (item) ? ; Ok (()) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { let this = self . project () ; let sink1_ready = this . sink1 . poll_flush (cx) ? . is_ready () ; let sink2_ready = this . sink2 . poll_flush (cx) ? . is_ready () ; let ready = sink1_ready && sink2_ready ; if ready { Poll :: Ready (Ok (())) } else { Poll :: Pending } } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { let this = self . project () ; let sink1_ready = this . sink1 . poll_close (cx) ? . is_ready () ; let sink2_ready = this . sink2 . poll_close (cx) ? . is_ready () ; let ready = sink1_ready && sink2_ready ; if ready { Poll :: Ready (Ok (())) } else { Poll :: Pending } } }
    };
}

impl_928!()