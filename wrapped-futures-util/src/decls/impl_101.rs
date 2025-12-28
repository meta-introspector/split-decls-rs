macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < Fut > FusedFuture for Shared < Fut > where Fut : Future , Fut :: Output : Clone , { fn is_terminated (& self) -> bool { self . inner . is_none () } }
    };
}

impl_101!();