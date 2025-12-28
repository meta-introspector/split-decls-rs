macro_rules! deps {
    () => {
        Data!();
        Any!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl Deref for Data { type Target = FnvHashMap < TypeId , Box < dyn Any + Sync + Send > > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_330!();