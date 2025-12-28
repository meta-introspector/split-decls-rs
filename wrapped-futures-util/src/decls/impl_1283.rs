macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! impl_1283 {
    () => {
        deps!();
        impl < T : Default > Default for Mutex < T > { fn default () -> Self { Self :: new (Default :: default ()) } }
    };
}

impl_1283!();