macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < C > ops :: Deref for Receiver < C > { type Target = C ; fn deref (& self) -> & C { & self . counter () . chan } }
    };
}

impl_57!();