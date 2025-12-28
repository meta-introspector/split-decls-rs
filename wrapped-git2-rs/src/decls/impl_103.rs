macro_rules! deps {
    () => {
        TreeUpdateBuilder!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl Default for TreeUpdateBuilder { fn default () -> Self { Self :: new () } }
    };
}

impl_103!();