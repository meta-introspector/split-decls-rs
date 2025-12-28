macro_rules! deps {
    () => {
        PathWrapper!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Deref for PathWrapper { type Target = Path ; fn deref (& self) -> & Self :: Target { self . path . deref () } }
    };
}

impl_12!();