macro_rules! deps {
    () => {
        FutureGroup!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl < T > Default for FutureGroup < T > { fn default () -> Self { Self :: new () } }
    };
}

impl_200!();