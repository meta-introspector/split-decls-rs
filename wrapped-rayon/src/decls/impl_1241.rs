macro_rules! deps {
    () => {
        IntoParallelIterator!();
        Iter!();
    };
}

macro_rules! impl_1241 {
    () => {
        deps!();
        impl < 'data , T : Sync > IntoParallelIterator for & 'data [T] { type Item = & 'data T ; type Iter = Iter < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { Iter { slice : self } } }
    };
}

impl_1241!();