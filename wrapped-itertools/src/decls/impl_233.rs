macro_rules! deps {
    () => {
        FlattenOk!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl < I , T , E > fmt :: Debug for FlattenOk < I , T , E > where I : Iterator < Item = Result < T , E > > + fmt :: Debug , T : IntoIterator , T :: IntoIter : fmt :: Debug , { debug_fmt_fields ! (FlattenOk , iter , inner_front , inner_back) ; }
    };
}

impl_233!();