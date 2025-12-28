macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl Default for Builder { fn default () -> Self { Self :: new () } }
    };
}

impl_233!()