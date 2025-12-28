macro_rules! deps {
    () => {
        Ready!();
        Sink!();
        Send!();
    };
}

macro_rules! impl_955 {
    () => {
        deps!();
        impl < Si : Sink < Item > + Unpin + ? Sized , Item > Future for Send < '_ , Si , Item > { type Output = Result < () , Si :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; if this . feed . is_item_pending () { ready ! (Pin :: new (& mut this . feed) . poll (cx)) ? ; debug_assert ! (! this . feed . is_item_pending ()) ; } ready ! (this . feed . sink_pin_mut () . poll_flush (cx)) ? ; Poll :: Ready (Ok (())) } }
    };
}

impl_955!()