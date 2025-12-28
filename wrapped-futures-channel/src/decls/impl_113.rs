macro_rules! deps {
    () => {
        Cancellation!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < T > Future for Cancellation < '_ , T > { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < () > { self . inner . poll_canceled (cx) } }
    };
}

impl_113!()