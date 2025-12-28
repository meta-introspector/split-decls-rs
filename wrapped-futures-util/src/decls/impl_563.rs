macro_rules! deps {
    () => {
        Ready!();
        Sink!();
        SplitSink!();
    };
}

macro_rules! impl_563 {
    () => {
        deps!();
        impl < S : Sink < Item > , Item > SplitSink < S , Item > { fn poll_flush_slot (mut inner : Pin < & mut S > , slot : & mut Option < Item > , cx : & mut Context < '_ > ,) -> Poll < Result < () , S :: Error > > { if slot . is_some () { ready ! (inner . as_mut () . poll_ready (cx)) ? ; Poll :: Ready (inner . start_send (slot . take () . unwrap ())) } else { Poll :: Ready (Ok (())) } } fn poll_lock_and_flush_slot (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Result < () , S :: Error > > { let this = & mut * self ; let mut inner = ready ! (this . lock . poll_lock (cx)) ; Self :: poll_flush_slot (inner . as_pin_mut () , & mut this . slot , cx) } }
    };
}

impl_563!()