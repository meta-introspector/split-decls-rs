macro_rules! deps {
    () => {
        TarjanScc!();
    };
}

macro_rules! impl_452 {
    () => {
        deps!();
        impl < N > Default for TarjanScc < N > { fn default () -> Self { Self :: new () } }
    };
}

impl_452!();