macro_rules! deps {
    () => {
        MacroCallLoc!();
        MacroCallId!();
    };
}

macro_rules! macro_20 {
    () => {
        deps!();
        impl_intern_lookup ! (ExpandDatabase , MacroCallId , MacroCallLoc , intern_macro_call , lookup_intern_macro_call) ;
    };
}

macro_20!()