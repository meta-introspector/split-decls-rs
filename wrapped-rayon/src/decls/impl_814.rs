macro_rules! deps {
    () => {
        Rev!();
    };
}

macro_rules! impl_814 {
    () => {
        deps!();
        impl < I > Rev < I > { # [doc = " Creates a new `Rev` iterator."] pub (super) fn new (base : I) -> Self { Rev { base } } }
    };
}

impl_814!()