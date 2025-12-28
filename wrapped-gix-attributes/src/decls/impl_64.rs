macro_rules! deps {
    () => {
        AttributeId!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl Default for AttributeId { fn default () -> Self { AttributeId (usize :: MAX) } }
    };
}

impl_64!();