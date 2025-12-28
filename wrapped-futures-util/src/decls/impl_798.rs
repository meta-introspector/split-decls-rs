macro_rules! deps {
    () => {
        PollNext!();
    };
}

macro_rules! impl_798 {
    () => {
        deps!();
        impl Default for PollNext { fn default () -> Self { Self :: Left } }
    };
}

impl_798!();