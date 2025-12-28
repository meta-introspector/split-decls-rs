macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < T > FusedStream for Receiver < T > { fn is_terminated (& self) -> bool { self . inner . is_none () } }
    };
}

impl_79!();