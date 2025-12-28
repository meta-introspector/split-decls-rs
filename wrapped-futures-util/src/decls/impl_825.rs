macro_rules! deps {
    () => {
        FuturesOrdered!();
    };
}

macro_rules! impl_825 {
    () => {
        deps!();
        impl < Fut : Future > Default for FuturesOrdered < Fut > { fn default () -> Self { Self :: new () } }
    };
}

impl_825!();