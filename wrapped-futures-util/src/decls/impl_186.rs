macro_rules! deps {
    () => {
        Ready!();
        Pending!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < T , F > Future for PollImmediate < F > where F : Future < Output = T > , { type Output = Option < T > ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < T > > { let mut this = self . project () ; let inner = this . future . as_mut () . as_pin_mut () . expect ("PollImmediate polled after completion") ; match inner . poll (cx) { Poll :: Ready (t) => { this . future . set (None) ; Poll :: Ready (Some (t)) } Poll :: Pending => Poll :: Ready (None) , } } }
    };
}

impl_186!()