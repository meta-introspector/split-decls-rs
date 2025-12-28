macro_rules! deps {
    () => {
        YieldNow!();
        Pending!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Future for YieldNow { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { if ! self . 0 { self . 0 = true ; cx . waker () . wake_by_ref () ; Poll :: Pending } else { Poll :: Ready (()) } } }
    };
}

impl_13!();