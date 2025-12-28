macro_rules! deps {
    () => {
        StoredProcMacroLoadResult!();
    };
}

macro_rules! CrateProcMacros {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct CrateProcMacros (StoredProcMacroLoadResult) ;
    };
}

CrateProcMacros!()