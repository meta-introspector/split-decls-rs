macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < T : ? Sized > ops :: Deref for Arc < T > { type Target = T ; fn deref (& self) -> & T { & self . value } }
    };
}

impl_241!();