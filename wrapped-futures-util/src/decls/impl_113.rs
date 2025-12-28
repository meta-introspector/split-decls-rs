macro_rules! impl_113 {
    () => {
        impl < Fut : TryFuture + FusedFuture > FusedFuture for IntoFuture < Fut > { fn is_terminated (& self) -> bool { self . future . is_terminated () } }
    };
}

impl_113!();