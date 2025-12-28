macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < R > Read < R > where R : io :: Read , { # [doc = " Create a new interruptible reader from `read`."] pub fn new (read : R) -> Self { Read { inner : gix_features :: interrupt :: Read { inner : read , should_interrupt : & IS_INTERRUPTED , } , } } # [doc = " Return the inner reader"] pub fn into_inner (self) -> R { self . inner . inner } }
    };
}

impl_5!();