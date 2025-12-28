macro_rules! deps {
    () => {
        Values!();
        Filters!();
    };
}

macro_rules! impl_568 {
    () => {
        deps!();
        impl < 'a > Deref for Filters < 'a > { type Target = Values < 'a > ; fn deref (& self) -> & Self :: Target { & self . values } }
    };
}

impl_568!()