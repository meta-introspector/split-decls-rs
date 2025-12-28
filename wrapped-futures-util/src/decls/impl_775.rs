macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_775 {
    () => {
        deps!();
        impl < T > FusedStream for Pending < T > { fn is_terminated (& self) -> bool { true } }
    };
}

impl_775!()