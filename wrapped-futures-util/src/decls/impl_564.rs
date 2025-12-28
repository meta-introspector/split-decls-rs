macro_rules! deps {
    () => {
        Ready!();
        SplitSink!();
        Sink!();
    };
}

macro_rules! impl_564 {
    () => {
        deps!();
        impl < S : Sink < Item > , Item > Sink < Item > for SplitSink < S , Item > { type Error = S :: Error ; fn poll_ready (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , S :: Error > > { loop { if self . slot . is_none () { return Poll :: Ready (Ok (())) ; } ready ! (self . as_mut () . poll_lock_and_flush_slot (cx)) ? ; } } fn start_send (mut self : Pin < & mut Self > , item : Item) -> Result < () , S :: Error > { self . slot = Some (item) ; Ok (()) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , S :: Error > > { let this = & mut * self ; let mut inner = ready ! (this . lock . poll_lock (cx)) ; ready ! (Self :: poll_flush_slot (inner . as_pin_mut () , & mut this . slot , cx)) ? ; inner . as_pin_mut () . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , S :: Error > > { let this = & mut * self ; let mut inner = ready ! (this . lock . poll_lock (cx)) ; ready ! (Self :: poll_flush_slot (inner . as_pin_mut () , & mut this . slot , cx)) ? ; inner . as_pin_mut () . poll_close (cx) } }
    };
}

impl_564!();