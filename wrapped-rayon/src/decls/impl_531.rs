macro_rules! deps {
    () => {
        Flatten!();
    };
}

macro_rules! impl_531 {
    () => {
        deps!();
        impl < I > Flatten < I > { # [doc = " Creates a new `Flatten` iterator."] pub (super) fn new (base : I) -> Self { Flatten { base } } }
    };
}

impl_531!();