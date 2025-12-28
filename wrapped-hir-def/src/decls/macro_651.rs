macro_rules! deps {
    () => {
        ModuleId!();
        Macro2Loc!();
    };
}

macro_rules! macro_651 {
    () => {
        deps!();
        impl_loc ! (Macro2Loc , id : MacroDef , container : ModuleId) ;
    };
}

macro_651!();