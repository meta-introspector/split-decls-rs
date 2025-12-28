macro_rules! deps {
    () => {
        AggregateError!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl < E > Deref for AggregateError < E > { type Target = Vec < E > ; fn deref (& self) -> & Self :: Target { & self . inner } }
    };
}

impl_330!()