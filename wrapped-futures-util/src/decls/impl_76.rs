macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < Fut : Future > Future for Remote < Fut > { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < () > { let this = self . project () ; if this . tx . as_mut () . unwrap () . poll_canceled (cx) . is_ready () && ! this . keep_running . load (Ordering :: SeqCst) { return Poll :: Ready (()) ; } let output = ready ! (this . future . poll (cx)) ; drop (this . tx . take () . unwrap () . send (output)) ; Poll :: Ready (()) } }
    };
}

impl_76!()