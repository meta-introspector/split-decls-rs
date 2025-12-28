macro_rules! deps {
    () => {
        InterleavePending!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < Fut : FusedFuture > FusedFuture for InterleavePending < Fut > { fn is_terminated (& self) -> bool { self . inner . is_terminated () } }
    };
}

impl_87!();