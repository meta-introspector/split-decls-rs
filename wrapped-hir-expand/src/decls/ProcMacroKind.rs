macro_rules! deps {
    () => {
        Attr!();
    };
}

macro_rules! ProcMacroKind {
    () => {
        deps!();
        # [derive (Copy , Clone , Eq , PartialEq , PartialOrd , Ord , Debug , Hash)] pub enum ProcMacroKind { CustomDerive , Bang , Attr , }
    };
}

ProcMacroKind!();