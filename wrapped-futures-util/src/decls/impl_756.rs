macro_rules! deps {
    () => {
        RepeatWith!();
    };
}

macro_rules! impl_756 {
    () => {
        deps!();
        impl < A , F : FnMut () -> A > FusedStream for RepeatWith < F > { fn is_terminated (& self) -> bool { false } }
    };
}

impl_756!();