macro_rules! deps {
    () => {
        MacroCallLoc!();
    };
}

macro_rules! MacroCallId {
    () => {
        deps!();
        # [salsa_macros :: interned (no_lifetime , debug , revisions = usize :: MAX)] # [doc (alias = "MacroFileId")] pub struct MacroCallId { pub loc : MacroCallLoc , }
    };
}

MacroCallId!()