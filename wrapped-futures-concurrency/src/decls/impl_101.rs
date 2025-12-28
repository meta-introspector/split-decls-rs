macro_rules! deps {
    () => {
        WakerVec!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl Default for WakerVec { fn default () -> Self { Self :: new (0) } }
    };
}

impl_101!();