macro_rules! deps {
    () => {
        AggregateError!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < E , const N : usize > Deref for AggregateError < E , N > { type Target = [E ; N] ; fn deref (& self) -> & Self :: Target { & self . inner } }
    };
}

impl_290!();