macro_rules! deps {
    () => {
        Serializer!();
    };
}

macro_rules! macro_118 {
    () => {
        deps!();
        deref_erased_serializer ! (< T > Serializer for & mut T where T : ? Sized + Serializer) ;
    };
}

macro_118!();