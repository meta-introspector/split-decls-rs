macro_rules! deps {
    () => {
        FilterOk!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < I , F , T , E > FusedIterator for FilterOk < I , F > where I : FusedIterator < Item = Result < T , E > > , F : FnMut (& T) -> bool , { }
    };
}

impl_125!()