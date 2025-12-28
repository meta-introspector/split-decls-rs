macro_rules! deps {
    () => {
        AssertUnmoved!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < Fut : FusedFuture > FusedFuture for AssertUnmoved < Fut > { fn is_terminated (& self) -> bool { self . inner . is_terminated () } }
    };
}

impl_73!()