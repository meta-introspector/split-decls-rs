macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < C > ops :: Deref for Sender < C > { type Target = C ; fn deref (& self) -> & C { & self . counter () . chan } }
    };
}

impl_53!()