macro_rules! deps {
    () => {
        Shake128!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl Default for Shake128 { fn default () -> Self { Self :: new () } }
    };
}

impl_200!()