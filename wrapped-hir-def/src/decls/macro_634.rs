macro_rules! deps {
    () => {
        TypeAliasLoc!();
    };
}

macro_rules! macro_634 {
    () => {
        deps!();
        impl_intern ! (TypeAliasId , TypeAliasLoc , intern_type_alias , lookup_intern_type_alias) ;
    };
}

macro_634!();