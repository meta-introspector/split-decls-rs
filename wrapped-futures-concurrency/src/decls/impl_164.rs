macro_rules! deps {
    () => {
        Limit!();
        ConcurrentStream!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < CS : ConcurrentStream > Limit < CS > { pub (crate) fn new (inner : CS , limit : Option < NonZeroUsize >) -> Self { Self { inner , limit } } }
    };
}

impl_164!();