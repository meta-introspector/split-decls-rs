macro_rules! deps {
    () => {
        Macro2Loc!();
    };
}

macro_rules! macro_78 {
    () => {
        deps!();
        impl_intern ! (Macro2Id , Macro2Loc , intern_macro2 , lookup_intern_macro2) ;
    };
}

macro_78!()