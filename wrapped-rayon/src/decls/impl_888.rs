macro_rules! deps {
    () => {
        TakeAnyWhile!();
    };
}

macro_rules! impl_888 {
    () => {
        deps!();
        impl < I , P > TakeAnyWhile < I , P > { # [doc = " Creates a new `TakeAnyWhile` iterator."] pub (super) fn new (base : I , predicate : P) -> Self { TakeAnyWhile { base , predicate } } }
    };
}

impl_888!();