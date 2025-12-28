macro_rules! deps {
    () => {
        MappedGenericSequence!();
        Mapped!();
        GenericSequence!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < 'a , T , U , S : MappedGenericSequence < T , U > > MappedGenericSequence < T , U > for & 'a S where & 'a S : GenericSequence < T > , S : GenericSequence < T , Length = < & 'a S as GenericSequence < T > > :: Length > , { type Mapped = < S as MappedGenericSequence < T , U > > :: Mapped ; }
    };
}

impl_106!();