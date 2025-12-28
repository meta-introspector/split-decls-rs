macro_rules! deps {
    () => {
        Positions!();
    };
}

macro_rules! impl_766 {
    () => {
        deps!();
        impl < I , P > Positions < I , P > { # [doc = " Create a new `Positions` iterator."] pub (super) fn new (base : I , predicate : P) -> Self { Positions { base , predicate } } }
    };
}

impl_766!()