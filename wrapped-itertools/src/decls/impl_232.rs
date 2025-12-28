macro_rules! deps {
    () => {
        FlattenOk!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl < I , T , E > Clone for FlattenOk < I , T , E > where I : Iterator < Item = Result < T , E > > + Clone , T : IntoIterator , T :: IntoIter : Clone , { clone_fields ! (iter , inner_front , inner_back) ; }
    };
}

impl_232!();