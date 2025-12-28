macro_rules! deps {
    () => {
        ModuleId!();
        MacroRules!();
        MacroRulesLoc!();
    };
}

macro_rules! macro_654 {
    () => {
        deps!();
        impl_loc ! (MacroRulesLoc , id : MacroRules , container : ModuleId) ;
    };
}

macro_654!()