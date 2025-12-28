macro_rules! deps {
    () => {
        Copied!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        impl < I > Copied < I > { # [doc = " Creates a new `Copied` iterator."] pub (super) fn new (base : I) -> Self { Copied { base } } }
    };
}

impl_380!()