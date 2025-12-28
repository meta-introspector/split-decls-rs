macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
        MappedGenericSequence!();
        Mapped!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < T , U , N : ArrayLength > MappedGenericSequence < T , U > for Box < GenericArray < T , N > > { type Mapped = Box < GenericArray < U , N > > ; }
    };
}

impl_67!()