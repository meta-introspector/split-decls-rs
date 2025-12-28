macro_rules! deps {
    () => {
        CompType!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl Default for CompType { fn default () -> Self { Self :: Normal } }
    };
}

impl_131!()