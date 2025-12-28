macro_rules! deps {
    () => {
        ID!();
    };
}

macro_rules! impl_763 {
    () => {
        deps!();
        impl DerefMut for ID { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_763!()