macro_rules! deps {
    () => {
        ID!();
    };
}

macro_rules! impl_762 {
    () => {
        deps!();
        impl Deref for ID { type Target = String ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_762!();