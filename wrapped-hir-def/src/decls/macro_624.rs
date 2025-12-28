macro_rules! deps {
    () => {
        EnumLoc!();
    };
}

macro_rules! macro_624 {
    () => {
        deps!();
        impl_intern ! (EnumId , EnumLoc , intern_enum , lookup_intern_enum) ;
    };
}

macro_624!()