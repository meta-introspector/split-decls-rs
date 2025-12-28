macro_rules! deps {
    () => {
        IntoParallelIterator!();
        IntoIter!();
        Iter!();
    };
}

macro_rules! impl_1117 {
    () => {
        deps!();
        impl < T : Send , E > IntoParallelIterator for Result < T , E > { type Item = T ; type Iter = IntoIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoIter { inner : self . ok () . into_par_iter () , } } }
    };
}

impl_1117!();