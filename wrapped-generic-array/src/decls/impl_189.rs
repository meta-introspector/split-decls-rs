macro_rules! deps {
    () => {
        MappedGenericSequence!();
        Mapped!();
        ArrayLength!();
        GenericSequence!();
        GenericArray!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < T , U , N : ArrayLength > MappedGenericSequence < T , U > for GenericArray < T , N > where GenericArray < U , N > : GenericSequence < U , Length = N > , { type Mapped = GenericArray < U , N > ; }
    };
}

impl_189!();