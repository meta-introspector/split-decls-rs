macro_rules! deps {
    () => {
        RebaseOptions!();
    };
}

macro_rules! impl_585 {
    () => {
        deps!();
        impl < 'cb > Default for RebaseOptions < 'cb > { fn default () -> Self { Self :: new () } }
    };
}

impl_585!();