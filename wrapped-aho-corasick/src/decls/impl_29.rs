macro_rules! deps {
    () => {
        NFA!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl private :: Sealed for crate :: nfa :: noncontiguous :: NFA { }
    };
}

impl_29!();