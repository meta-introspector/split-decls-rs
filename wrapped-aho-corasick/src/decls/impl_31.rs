macro_rules! deps {
    () => {
        DFA!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl private :: Sealed for crate :: dfa :: DFA { }
    };
}

impl_31!()