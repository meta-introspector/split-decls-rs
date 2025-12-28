macro_rules! deps {
    () => {
        CrateProcMacros!();
    };
}

macro_rules! ProcMacrosBuilder {
    () => {
        deps!();
        # [derive (Default , Debug)] pub struct ProcMacrosBuilder (FxHashMap < CrateBuilderId , Arc < CrateProcMacros > >) ;
    };
}

ProcMacrosBuilder!()