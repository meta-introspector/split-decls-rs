macro_rules! deps {
    () => {
        AlwaysReady!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < T , F : Fn () -> T > FusedFuture for AlwaysReady < T , F > { fn is_terminated (& self) -> bool { false } }
    };
}

impl_205!();