macro_rules! deps {
    () => {
        TraitLoc!();
    };
}

macro_rules! macro_631 {
    () => {
        deps!();
        impl_intern ! (TraitId , TraitLoc , intern_trait , lookup_intern_trait) ;
    };
}

macro_631!();