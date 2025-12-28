macro_rules! deps {
    () => {
        ProcMacroDef!();
        ProcMacroDerive!();
    };
}

macro_rules! ProcMacro {
    () => {
        deps!();
        enum ProcMacro { Derive (ProcMacroDerive) , Attr (ProcMacroDef) , Bang (ProcMacroDef) , }
    };
}

ProcMacro!();