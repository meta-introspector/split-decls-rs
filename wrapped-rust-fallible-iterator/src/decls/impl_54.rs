macro_rules! deps {
    () => {
        IntoFallibleIterator!();
        FallibleIterator!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < I > IntoFallibleIterator for I where I : FallibleIterator , { type Item = I :: Item ; type Error = I :: Error ; type IntoFallibleIter = I ; # [inline] fn into_fallible_iter (self) -> I { self } }
    };
}

impl_54!()