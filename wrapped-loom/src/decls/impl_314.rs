macro_rules! deps {
    () => {
        RwLock!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl < T : Default > Default for RwLock < T > { # [doc = " Creates a `RwLock<T>`, with the `Default` value for T."] fn default () -> Self { Self :: new (Default :: default ()) } }
    };
}

impl_314!()