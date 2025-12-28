macro_rules! deps {
    () => {
        ImplLoc!();
    };
}

macro_rules! macro_64 {
    () => {
        deps!();
        impl_intern ! (ImplId , ImplLoc , intern_impl , lookup_intern_impl) ;
    };
}

macro_64!()