macro_rules! deps {
    () => {
        Interface!();
        ScopedInterface!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl < T : Interface > core :: ops :: Deref for ScopedInterface < '_ , T > { type Target = T ; fn deref (& self) -> & T { & self . interface } }
    };
}

impl_195!()