macro_rules! deps {
    () => {
        ExternBlockLoc!();
    };
}

macro_rules! macro_643 {
    () => {
        deps!();
        impl_intern ! (ExternBlockId , ExternBlockLoc , intern_extern_block , lookup_intern_extern_block) ;
    };
}

macro_643!();