macro_rules! deps {
    () => {
        Serializer!();
    };
}

macro_rules! macro_119 {
    () => {
        deps!();
        deref_erased_serializer ! (< T > Serializer for Box < T > where T : ? Sized + Serializer) ;
    };
}

macro_119!()