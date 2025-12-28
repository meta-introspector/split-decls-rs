macro_rules! deps {
    () => {
        OpaqueCursor!();
    };
}

macro_rules! impl_719 {
    () => {
        deps!();
        impl < T > Deref for OpaqueCursor < T > { type Target = T ; # [inline] fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_719!();