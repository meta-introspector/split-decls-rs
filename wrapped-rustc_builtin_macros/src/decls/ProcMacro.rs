macro_rules! deps {
    () => {
        ProcMacroDerive!();
        ProcMacroDef!();
    };
}

macro_rules! ProcMacro {
    () => {
        deps!();
        enum ProcMacro { Derive (ProcMacroDerive) , Attr (ProcMacroDef) , Bang (ProcMacroDef) , }
    };
}

ProcMacro!()