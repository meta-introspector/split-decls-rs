macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T , U , N : ArrayLength > MappedGenericSequence < T , U > for GenericArray < T , N > where GenericArray < U , N > : GenericSequence < U , Length = N > , { type Mapped = GenericArray < U , N > ; }
    };
}

impl_43!()