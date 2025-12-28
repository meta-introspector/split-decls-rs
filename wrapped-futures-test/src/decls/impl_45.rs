macro_rules! deps {
    () => {
        PendingOnce!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < Fut : Future > Future for PendingOnce < Fut > { type Output = Fut :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if * this . polled_before { this . future . poll (cx) } else { * this . polled_before = true ; cx . waker () . wake_by_ref () ; Poll :: Pending } } }
    };
}

impl_45!();