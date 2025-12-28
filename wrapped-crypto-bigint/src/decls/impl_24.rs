macro_rules! deps {
    () => {
        Checked!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T > Checked < T > { # [doc = " Create a new checked arithmetic wrapper for the given value."] pub fn new (val : T) -> Self { Self (CtOption :: new (val , Choice :: from (1))) } }
    };
}

impl_24!();