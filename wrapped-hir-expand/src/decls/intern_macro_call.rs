macro_rules! deps {
    () => {
        ExpandDatabase!();
        MacroCallId!();
        MacroCallLoc!();
    };
}

macro_rules! intern_macro_call {
    () => {
        deps!();
        fn intern_macro_call (db : & dyn ExpandDatabase , macro_call : MacroCallLoc) -> MacroCallId { MacroCallId :: new (db , macro_call) }
    };
}

intern_macro_call!();