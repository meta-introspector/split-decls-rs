macro_rules! deps {
    () => {
        IntoParallelIterator!();
        Iter!();
    };
}

macro_rules! impl_1121 {
    () => {
        deps!();
        impl < 'a , T : Sync , E > IntoParallelIterator for & 'a Result < T , E > { type Item = & 'a T ; type Iter = Iter < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { Iter { inner : self . as_ref () . ok () . into_par_iter () , } } }
    };
}

impl_1121!()