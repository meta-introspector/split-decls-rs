macro_rules! deps {
    () => {
        ArenaMapIter!();
        Idx!();
        ArenaMap!();
        IntoIter!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T , V > IntoIterator for ArenaMap < Idx < T > , V > { type Item = (Idx < T > , V) ; type IntoIter = ArenaMapIter < Idx < T > , V > ; fn into_iter (self) -> Self :: IntoIter { let iter = self . v . into_iter () . enumerate () ; Self :: IntoIter { iter , _ty : PhantomData } } }
    };
}

impl_8!()