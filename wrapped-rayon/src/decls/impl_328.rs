macro_rules! deps {
    () => {
        Cloned!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl < I > Cloned < I > { # [doc = " Creates a new `Cloned` iterator."] pub (super) fn new (base : I) -> Self { Cloned { base } } }
    };
}

impl_328!()