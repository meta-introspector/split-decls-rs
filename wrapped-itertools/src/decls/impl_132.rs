macro_rules! deps {
    () => {
        FilterMapOk!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < I , F , T , U , E > FusedIterator for FilterMapOk < I , F > where I : FusedIterator < Item = Result < T , E > > , F : FnMut (T) -> Option < U > , { }
    };
}

impl_132!()