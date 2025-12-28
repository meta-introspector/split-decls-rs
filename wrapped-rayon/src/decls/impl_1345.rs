macro_rules! deps {
    () => {
        Iter!();
        IterMut!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_1345 {
    () => {
        deps!();
        impl < 'data , T : Send + 'data > IntoParallelIterator for & 'data mut Vec < T > { type Item = & 'data mut T ; type Iter = IterMut < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { < & mut [T] > :: into_par_iter (self) } }
    };
}

impl_1345!();