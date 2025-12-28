macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl Deref for Id < '_ > { type Target = oid ; fn deref (& self) -> & Self :: Target { & self . inner } }
    };
}

impl_143!()