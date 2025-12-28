macro_rules! deps {
    () => {
        IntoParallelIterator!();
        Iter!();
    };
}

macro_rules! impl_1344 {
    () => {
        deps!();
        impl < 'data , T : Sync + 'data > IntoParallelIterator for & 'data Vec < T > { type Item = & 'data T ; type Iter = Iter < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { < & [T] > :: into_par_iter (self) } }
    };
}

impl_1344!()