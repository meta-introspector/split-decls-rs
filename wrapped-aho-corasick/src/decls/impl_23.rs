macro_rules! deps {
    () => {
        AcAutomaton!();
        Automaton!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < A > AcAutomaton for A where A : Automaton + Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static { }
    };
}

impl_23!()