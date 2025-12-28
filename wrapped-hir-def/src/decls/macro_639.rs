macro_rules! deps {
    () => {
        UseLoc!();
    };
}

macro_rules! macro_639 {
    () => {
        deps!();
        impl_intern ! (UseId , UseLoc , intern_use , lookup_intern_use) ;
    };
}

macro_639!()