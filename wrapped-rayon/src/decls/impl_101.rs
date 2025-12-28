macro_rules! deps {
    () => {
        IntoParallelIterator!();
        IterMut!();
        Iter!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < 'a , T : Send > IntoParallelIterator for & 'a mut VecDeque < T > { type Item = & 'a mut T ; type Iter = IterMut < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { let (a , b) = self . as_mut_slices () ; IterMut { inner : a . into_par_iter () . chain (b) , } } }
    };
}

impl_101!();