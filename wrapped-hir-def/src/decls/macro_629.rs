macro_rules! deps {
    () => {
        StaticLoc!();
    };
}

macro_rules! macro_629 {
    () => {
        deps!();
        impl_intern ! (StaticId , StaticLoc , intern_static , lookup_intern_static) ;
    };
}

macro_629!()