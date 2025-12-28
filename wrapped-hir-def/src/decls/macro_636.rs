macro_rules! deps {
    () => {
        ImplLoc!();
    };
}

macro_rules! macro_636 {
    () => {
        deps!();
        impl_intern ! (ImplId , ImplLoc , intern_impl , lookup_intern_impl) ;
    };
}

macro_636!()