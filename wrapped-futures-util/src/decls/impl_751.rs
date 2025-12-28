macro_rules! deps {
    () => {
        Repeat!();
    };
}

macro_rules! impl_751 {
    () => {
        deps!();
        impl < T > FusedStream for Repeat < T > where T : Clone , { fn is_terminated (& self) -> bool { false } }
    };
}

impl_751!();