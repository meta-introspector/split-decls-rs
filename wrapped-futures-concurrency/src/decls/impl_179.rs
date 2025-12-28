macro_rules! deps {
    () => {
        Take!();
        ConcurrentStream!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < CS : ConcurrentStream > Take < CS > { pub (crate) fn new (inner : CS , limit : usize) -> Self { Self { inner , limit } } }
    };
}

impl_179!();