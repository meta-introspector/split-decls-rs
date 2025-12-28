macro_rules! deps {
    () => {
        ComObjectInner!();
        StaticComObject!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < T > core :: ops :: Deref for StaticComObject < T > where T : ComObjectInner , { type Target = T :: Outer ; fn deref (& self) -> & Self :: Target { & self . outer } }
    };
}

impl_129!()