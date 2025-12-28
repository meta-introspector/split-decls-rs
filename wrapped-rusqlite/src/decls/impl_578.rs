macro_rules! deps {
    () => {
        Values!();
        Inserts!();
    };
}

macro_rules! impl_578 {
    () => {
        deps!();
        impl < 'a > Deref for Inserts < 'a > { type Target = Values < 'a > ; fn deref (& self) -> & Self :: Target { & self . values } }
    };
}

impl_578!();