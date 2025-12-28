macro_rules! deps {
    () => {
        AssertUnmoved!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < Fut : Future > Future for AssertUnmoved < Fut > { type Output = Fut :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . poll_with (| f | f . poll (cx)) } }
    };
}

impl_72!()