macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! impl_299 {
    () => {
        deps!();
        impl < T : ? Sized + Default > Default for Mutex < T > { # [doc = " Creates a `Mutex<T>`, with the `Default` value for T."] fn default () -> Self { Self :: new (Default :: default ()) } }
    };
}

impl_299!();