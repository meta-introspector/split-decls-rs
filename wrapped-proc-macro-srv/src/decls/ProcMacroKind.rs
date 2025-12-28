macro_rules! ProcMacroKind {
    () => {
        # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub enum ProcMacroKind { CustomDerive , Attr , Bang , }
    };
}

ProcMacroKind!();