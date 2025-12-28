macro_rules! deps {
    () => {
        ProcMacrosBuilder!();
    };
}

macro_rules! ChangeWithProcMacros {
    () => {
        deps!();
        # [derive (Debug , Default)] pub struct ChangeWithProcMacros { pub source_change : FileChange , pub proc_macros : Option < ProcMacrosBuilder > , }
    };
}

ChangeWithProcMacros!()