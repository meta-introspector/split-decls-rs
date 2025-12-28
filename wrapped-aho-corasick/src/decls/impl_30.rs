macro_rules! deps {
    () => {
        NFA!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl private :: Sealed for crate :: nfa :: contiguous :: NFA { }
    };
}

impl_30!()