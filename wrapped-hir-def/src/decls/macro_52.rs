macro_rules! deps {
    () => {
        EnumLoc!();
    };
}

macro_rules! macro_52 {
    () => {
        deps!();
        impl_intern ! (EnumId , EnumLoc , intern_enum , lookup_intern_enum) ;
    };
}

macro_52!()