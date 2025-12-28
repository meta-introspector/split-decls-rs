macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_762 {
    () => {
        deps!();
        impl < T > FusedStream for Empty < T > { fn is_terminated (& self) -> bool { true } }
    };
}

impl_762!()