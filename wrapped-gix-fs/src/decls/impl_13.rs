macro_rules! deps {
    () => {
        FileSnapshot!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T : std :: fmt :: Debug > Deref for FileSnapshot < T > { type Target = T ; fn deref (& self) -> & Self :: Target { & self . value } }
    };
}

impl_13!()