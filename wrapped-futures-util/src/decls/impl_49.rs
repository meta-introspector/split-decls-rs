macro_rules! impl_49 {
    () => {
        impl < Fut : Future > FusedFuture for Fuse < Fut > { fn is_terminated (& self) -> bool { self . inner . is_none () } }
    };
}

impl_49!()