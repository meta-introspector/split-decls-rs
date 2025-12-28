macro_rules! deps {
    () => {
        FlattenIter!();
    };
}

macro_rules! impl_541 {
    () => {
        deps!();
        impl < I > FlattenIter < I > { # [doc = " Creates a new `FlattenIter` iterator."] pub (super) fn new (base : I) -> Self { FlattenIter { base } } }
    };
}

impl_541!()