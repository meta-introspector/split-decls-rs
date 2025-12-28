macro_rules! deps {
    () => {
        MacroRulesLoc!();
        MacroRules!();
        ModuleId!();
    };
}

macro_rules! macro_654 {
    () => {
        deps!();
        impl_loc ! (MacroRulesLoc , id : MacroRules , container : ModuleId) ;
    };
}

macro_654!();