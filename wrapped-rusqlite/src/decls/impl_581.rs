macro_rules! deps {
    () => {
        Values!();
        Updates!();
    };
}

macro_rules! impl_581 {
    () => {
        deps!();
        impl < 'a > Deref for Updates < 'a > { type Target = Values < 'a > ; fn deref (& self) -> & Self :: Target { & self . values } }
    };
}

impl_581!()