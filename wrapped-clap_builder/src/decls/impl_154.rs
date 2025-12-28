macro_rules! deps {
    () => {
        ValueRange!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl Default for ValueRange { fn default () -> Self { Self :: SINGLE } }
    };
}

impl_154!();