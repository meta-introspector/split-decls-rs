macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < 'a , T > Default for & 'a mut RevSlice < T > { fn default () -> Self { Self :: from (& mut []) } }
    };
}

impl_60!();