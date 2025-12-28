macro_rules! deps {
    () => {
        ProcMacroAutoTraits!();
    };
}

macro_rules! MARKER {
    () => {
        deps!();
        pub (crate) const MARKER : ProcMacroAutoTraits = ProcMacroAutoTraits (PhantomData) ;
    };
}

MARKER!();