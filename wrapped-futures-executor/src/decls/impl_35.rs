macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl Default for ThreadPoolBuilder { fn default () -> Self { Self :: new () } }
    };
}

impl_35!();