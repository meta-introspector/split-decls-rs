macro_rules! deps {
    () => {
        ExternCrateLoc!();
    };
}

macro_rules! macro_641 {
    () => {
        deps!();
        impl_intern ! (ExternCrateId , ExternCrateLoc , intern_extern_crate , lookup_intern_extern_crate) ;
    };
}

macro_641!()