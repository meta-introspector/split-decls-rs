macro_rules! deps {
    () => {
        SkipAnyWhile!();
    };
}

macro_rules! impl_837 {
    () => {
        deps!();
        impl < I , P > SkipAnyWhile < I , P > { # [doc = " Creates a new `SkipAnyWhile` iterator."] pub (super) fn new (base : I , predicate : P) -> Self { SkipAnyWhile { base , predicate } } }
    };
}

impl_837!();