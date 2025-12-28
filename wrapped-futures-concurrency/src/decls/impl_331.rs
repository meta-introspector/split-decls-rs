macro_rules! deps {
    () => {
        AggregateError!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl < E > DerefMut for AggregateError < E > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . inner } }
    };
}

impl_331!()