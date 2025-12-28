macro_rules! deps {
    () => {
        AcAutomaton!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl crate :: automaton :: private :: Sealed for Arc < dyn AcAutomaton > { }
    };
}

impl_24!();