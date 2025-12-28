macro_rules! deps {
    () => {
        Json!();
    };
}

macro_rules! impl_775 {
    () => {
        deps!();
        impl < T > Deref for Json < T > { type Target = T ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_775!()