macro_rules! deps {
    () => {
        Keyed!();
        FutureGroup!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < F : Future > Deref for Keyed < F > { type Target = FutureGroup < F > ; fn deref (& self) -> & Self :: Target { & self . group } }
    };
}

impl_209!();