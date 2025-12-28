macro_rules! deps {
    () => {
        ExternCrateLoc!();
    };
}

macro_rules! macro_69 {
    () => {
        deps!();
        impl_intern ! (ExternCrateId , ExternCrateLoc , intern_extern_crate , lookup_intern_extern_crate) ;
    };
}

macro_69!()