macro_rules! deps {
    () => {
        OpaqueCursor!();
    };
}

macro_rules! impl_720 {
    () => {
        deps!();
        impl < T > DerefMut for OpaqueCursor < T > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_720!()