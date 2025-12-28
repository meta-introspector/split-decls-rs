macro_rules! deps {
    () => {
        BodyRef!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Deref for BodyRef < '_ > { type Target = BStr ; fn deref (& self) -> & Self :: Target { self . body_without_trailer } }
    };
}

impl_10!();