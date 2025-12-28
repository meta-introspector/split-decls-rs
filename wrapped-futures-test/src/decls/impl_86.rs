macro_rules! deps {
    () => {
        InterleavePending!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < Fut : Future > Future for InterleavePending < Fut > { type Output = Fut :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . poll_with (cx , Fut :: poll) } }
    };
}

impl_86!()