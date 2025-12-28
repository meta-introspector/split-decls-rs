macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_467 {
    () => {
        deps!();
        impl < St , Fut , F > Future for TryForEach < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : TryFuture < Ok = () > , { type Output = Result < () , Fut :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { if let Some (fut) = this . future . as_mut () . as_pin_mut () { ready ! (fut . try_poll (cx)) ? ; this . future . set (None) ; } else { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (e) => this . future . set (Some ((this . f) (e))) , None => break , } } } Poll :: Ready (Ok (())) } }
    };
}

impl_467!()