macro_rules! deps {
    () => {
        IntoParallelIterator!();
        IterMut!();
        Iter!();
    };
}

macro_rules! impl_1047 {
    () => {
        deps!();
        impl < 'a , T : Send > IntoParallelIterator for & 'a mut Option < T > { type Item = & 'a mut T ; type Iter = IterMut < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { IterMut { inner : self . as_mut () . into_par_iter () , } } }
    };
}

impl_1047!()