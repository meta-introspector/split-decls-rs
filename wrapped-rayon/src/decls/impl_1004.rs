macro_rules! deps {
    () => {
        WhileSome!();
    };
}

macro_rules! impl_1004 {
    () => {
        deps!();
        impl < I > WhileSome < I > { # [doc = " Creates a new `WhileSome` iterator."] pub (super) fn new (base : I) -> Self { WhileSome { base } } }
    };
}

impl_1004!()