macro_rules! deps {
    () => {
        Enumerate!();
    };
}

macro_rules! impl_402 {
    () => {
        deps!();
        impl < I > Enumerate < I > { # [doc = " Creates a new `Enumerate` iterator."] pub (super) fn new (base : I) -> Self { Enumerate { base } } }
    };
}

impl_402!()