macro_rules! deps {
    () => {
        Feed!();
        Sink!();
        Ready!();
    };
}

macro_rules! impl_933 {
    () => {
        deps!();
        impl < Si : Sink < Item > + Unpin + ? Sized , Item > Future for Feed < '_ , Si , Item > { type Output = Result < () , Si :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . get_mut () ; let mut sink = Pin :: new (& mut this . sink) ; ready ! (sink . as_mut () . poll_ready (cx)) ? ; let item = this . item . take () . expect ("polled Feed after completion") ; sink . as_mut () . start_send (item) ? ; Poll :: Ready (Ok (())) } }
    };
}

impl_933!();