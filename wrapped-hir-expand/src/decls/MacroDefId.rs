macro_rules! deps {
    () => {
        MacroDefKind!();
    };
}

macro_rules! MacroDefId {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct MacroDefId { pub krate : Crate , pub edition : Edition , pub kind : MacroDefKind , pub local_inner : bool , pub allow_internal_unsafe : bool , }
    };
}

MacroDefId!();