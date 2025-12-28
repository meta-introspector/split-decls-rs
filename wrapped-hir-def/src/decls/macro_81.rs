macro_rules! deps {
    () => {
        MacroRulesLoc!();
    };
}

macro_rules! macro_81 {
    () => {
        deps!();
        impl_intern ! (MacroRulesId , MacroRulesLoc , intern_macro_rules , lookup_intern_macro_rules) ;
    };
}

macro_81!()