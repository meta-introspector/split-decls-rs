macro_rules! impl_187 {
    () => {
        impl < T : Future > FusedFuture for PollImmediate < T > { fn is_terminated (& self) -> bool { self . future . is_none () } }
    };
}

impl_187!();