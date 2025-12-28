macro_rules! impl_770 {
    () => {
        impl < Fut : Future > FusedStream for Once < Fut > { fn is_terminated (& self) -> bool { self . future . is_none () } }
    };
}

impl_770!();