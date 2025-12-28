macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl Default for Builder { fn default () -> Self { Builder :: new () } }
    };
}

impl_42!()