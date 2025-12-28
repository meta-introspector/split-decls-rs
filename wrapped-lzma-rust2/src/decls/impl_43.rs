macro_rules! deps {
    () => {
        LzEncoder!();
        LzEncoderData!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl Deref for LzEncoder { type Target = LzEncoderData ; fn deref (& self) -> & Self :: Target { & self . data } }
    };
}

impl_43!();