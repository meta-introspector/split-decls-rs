macro_rules! deps {
    () => {
        LocalPool!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Default for LocalPool { fn default () -> Self { Self :: new () } }
    };
}

impl_10!()