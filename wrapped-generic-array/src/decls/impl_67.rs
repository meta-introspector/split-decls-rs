macro_rules! deps {
    () => {
        Mapped!();
        ArrayLength!();
        MappedGenericSequence!();
        GenericArray!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < T , U , N : ArrayLength > MappedGenericSequence < T , U > for Box < GenericArray < T , N > > { type Mapped = Box < GenericArray < U , N > > ; }
    };
}

impl_67!();