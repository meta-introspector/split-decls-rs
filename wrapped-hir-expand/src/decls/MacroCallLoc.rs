macro_rules! deps {
    () => {
        MacroDefId!();
        MacroCallKind!();
    };
}

macro_rules! MacroCallLoc {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct MacroCallLoc { pub def : MacroDefId , pub krate : Crate , pub kind : MacroCallKind , pub ctxt : SyntaxContext , }
    };
}

MacroCallLoc!()