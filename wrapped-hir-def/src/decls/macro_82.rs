macro_rules! deps {
    () => {
        MacroRulesLoc!();
        ModuleId!();
    };
}

macro_rules! macro_82 {
    () => {
        deps!();
        impl_loc ! (MacroRulesLoc , id : MacroRules , container : ModuleId) ;
    };
}

macro_82!()