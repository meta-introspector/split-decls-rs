macro_rules! deps {
    () => {
        Notify!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl Default for Notify { fn default () -> Self { Self :: new () } }
    };
}

impl_308!();