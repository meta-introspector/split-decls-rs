macro_rules! deps {
    () => {
        FunctionalSequence!();
        ArrayLength!();
        GenericArray!();
        GenericSequence!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < T , N : ArrayLength > FunctionalSequence < T > for Box < GenericArray < T , N > > where Self : GenericSequence < T , Item = T , Length = N > { }
    };
}

impl_68!()