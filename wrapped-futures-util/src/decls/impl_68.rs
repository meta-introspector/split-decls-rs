macro_rules! deps {
    () => {
        Send!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < Fut > Future for CatchUnwind < Fut > where Fut : Future + UnwindSafe , { type Output = Result < Fut :: Output , Box < dyn Any + Send > > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let f = self . project () . future ; catch_unwind (AssertUnwindSafe (| | f . poll (cx))) ? . map (Ok) } }
    };
}

impl_68!()