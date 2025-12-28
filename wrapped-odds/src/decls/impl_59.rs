macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < 'a , T > Default for & 'a RevSlice < T > { fn default () -> Self { Self :: from (& []) } }
    };
}

impl_59!()