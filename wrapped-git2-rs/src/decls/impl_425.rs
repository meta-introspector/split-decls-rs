macro_rules! deps {
    () => {
        MergeOptions!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        impl Default for MergeOptions { fn default () -> Self { Self :: new () } }
    };
}

impl_425!();