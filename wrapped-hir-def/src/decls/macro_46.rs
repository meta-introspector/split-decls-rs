macro_rules! deps {
    () => {
        StructLoc!();
    };
}

macro_rules! macro_46 {
    () => {
        deps!();
        impl_intern ! (StructId , StructLoc , intern_struct , lookup_intern_struct) ;
    };
}

macro_46!()