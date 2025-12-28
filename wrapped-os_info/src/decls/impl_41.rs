macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl Default for Type { fn default () -> Self { Type :: Unknown } }
    };
}

impl_41!()