macro_rules! deps {
    () => {
        MacroId!();
    };
}

macro_rules! macro_693 {
    () => {
        deps!();
        impl_from ! (Macro2Id , MacroRulesId , ProcMacroId for MacroId) ;
    };
}

macro_693!();