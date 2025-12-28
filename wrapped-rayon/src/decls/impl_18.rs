macro_rules! deps {
    () => {
        IntoParallelIterator!();
        Iter!();
        IterMut!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < 'data , T : Send + 'data , const N : usize > IntoParallelIterator for & 'data mut [T ; N] { type Item = & 'data mut T ; type Iter = IterMut < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { < & mut [T] > :: into_par_iter (self) } }
    };
}

impl_18!();