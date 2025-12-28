macro_rules! deps {
    () => {
        Compress!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl Default for Compress { fn default () -> Self { Self :: new () } }
    };
}

impl_104!();