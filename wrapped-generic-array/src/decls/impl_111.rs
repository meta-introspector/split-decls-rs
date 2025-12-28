macro_rules! deps {
    () => {
        FunctionalSequence!();
        GenericSequence!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < 'a , T , S : GenericSequence < T > > FunctionalSequence < T > for & 'a S where & 'a S : GenericSequence < T > { }
    };
}

impl_111!()