macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < T > FusedFuture for Pending < T > { fn is_terminated (& self) -> bool { true } }
    };
}

impl_153!()