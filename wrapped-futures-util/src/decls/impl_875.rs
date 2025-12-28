macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_875 {
    () => {
        deps!();
        impl < Fut > Default for FuturesUnordered < Fut > { fn default () -> Self { Self :: new () } }
    };
}

impl_875!();