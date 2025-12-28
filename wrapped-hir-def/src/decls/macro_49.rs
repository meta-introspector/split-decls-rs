macro_rules! deps {
    () => {
        UnionLoc!();
    };
}

macro_rules! macro_49 {
    () => {
        deps!();
        impl_intern ! (UnionId , UnionLoc , intern_union , lookup_intern_union) ;
    };
}

macro_49!()