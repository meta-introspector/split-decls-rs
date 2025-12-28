macro_rules! deps {
    () => {
        ConstLoc!();
    };
}

macro_rules! macro_627 {
    () => {
        deps!();
        impl_intern ! (ConstId , ConstLoc , intern_const , lookup_intern_const) ;
    };
}

macro_627!();