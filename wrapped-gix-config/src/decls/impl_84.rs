macro_rules! deps {
    () => {
        Body!();
        Section!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < 'a > Deref for Section < 'a > { type Target = Body < 'a > ; fn deref (& self) -> & Self :: Target { & self . body } }
    };
}

impl_84!();