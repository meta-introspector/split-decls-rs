macro_rules! deps {
    () => {
        CrateProcMacros!();
    };
}

macro_rules! ProcMacros {
    () => {
        deps!();
        # [derive (Default , Debug)] pub struct ProcMacros (FxHashMap < Crate , Arc < CrateProcMacros > >) ;
    };
}

ProcMacros!();