macro_rules! deps {
    () => {
        MappedGenericSequence!();
        GenericSequence!();
        Mapped!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < 'a , T , U , S : MappedGenericSequence < T , U > > MappedGenericSequence < T , U > for & 'a mut S where & 'a mut S : GenericSequence < T > , S : GenericSequence < T , Length = < & 'a mut S as GenericSequence < T > > :: Length > , { type Mapped = < S as MappedGenericSequence < T , U > > :: Mapped ; }
    };
}

impl_107!();