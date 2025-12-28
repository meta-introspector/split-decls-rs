macro_rules! deps {
    () => {
        FetchOptions!();
    };
}

macro_rules! impl_653 {
    () => {
        deps!();
        impl < 'cb > Default for FetchOptions < 'cb > { fn default () -> Self { Self :: new () } }
    };
}

impl_653!();