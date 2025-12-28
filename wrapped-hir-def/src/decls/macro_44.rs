macro_rules! deps {
    () => {
        FunctionLoc!();
    };
}

macro_rules! macro_44 {
    () => {
        deps!();
        impl_intern ! (FunctionId , FunctionLoc , intern_function , lookup_intern_function) ;
    };
}

macro_44!()