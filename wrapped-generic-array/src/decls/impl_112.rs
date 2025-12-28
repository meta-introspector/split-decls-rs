macro_rules! deps {
    () => {
        FunctionalSequence!();
        GenericSequence!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < 'a , T , S : GenericSequence < T > > FunctionalSequence < T > for & 'a mut S where & 'a mut S : GenericSequence < T > { }
    };
}

impl_112!()