macro_rules! deps {
    () => {
        PendingOnce!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < Fut : FusedFuture > FusedFuture for PendingOnce < Fut > { fn is_terminated (& self) -> bool { self . polled_before && self . future . is_terminated () } }
    };
}

impl_46!()