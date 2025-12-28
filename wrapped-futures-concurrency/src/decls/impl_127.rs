macro_rules! deps {
    () => {
        Enumerate!();
        ConcurrentStream!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < CS : ConcurrentStream > Enumerate < CS > { pub (crate) fn new (inner : CS) -> Self { Self { inner } } }
    };
}

impl_127!()