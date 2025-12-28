macro_rules! deps {
    () => {
        Macro2Loc!();
        ModuleId!();
    };
}

macro_rules! macro_79 {
    () => {
        deps!();
        impl_loc ! (Macro2Loc , id : MacroDef , container : ModuleId) ;
    };
}

macro_79!()