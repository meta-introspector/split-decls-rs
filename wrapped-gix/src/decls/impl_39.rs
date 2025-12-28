macro_rules! deps {
    () => {
        AttributeStack!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl DerefMut for AttributeStack < '_ > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . inner } }
    };
}

impl_39!()