macro_rules! deps {
    () => {
        AggregateError!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl < E , const N : usize > DerefMut for AggregateError < E , N > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . inner } }
    };
}

impl_291!();