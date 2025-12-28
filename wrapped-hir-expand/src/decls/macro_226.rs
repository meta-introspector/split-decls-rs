macro_rules! deps {
    () => {
        ExpandDatabase!();
        MacroCallId!();
        MacroCallLoc!();
    };
}

macro_rules! macro_226 {
    () => {
        deps!();
        impl_intern_lookup ! (ExpandDatabase , MacroCallId , MacroCallLoc , intern_macro_call , lookup_intern_macro_call) ;
    };
}

macro_226!();