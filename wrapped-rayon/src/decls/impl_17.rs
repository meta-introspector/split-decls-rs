macro_rules! deps {
    () => {
        IntoParallelIterator!();
        Iter!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'data , T : Sync + 'data , const N : usize > IntoParallelIterator for & 'data [T ; N] { type Item = & 'data T ; type Iter = Iter < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { < & [T] > :: into_par_iter (self) } }
    };
}

impl_17!()