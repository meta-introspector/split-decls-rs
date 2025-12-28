macro_rules! deps {
    () => {
        MacroCallId!();
        ExpandDatabase!();
        MacroCallLoc!();
    };
}

macro_rules! lookup_intern_macro_call {
    () => {
        deps!();
        fn lookup_intern_macro_call (db : & dyn ExpandDatabase , macro_call : MacroCallId) -> MacroCallLoc { macro_call . loc (db) }
    };
}

lookup_intern_macro_call!()