macro_rules! deps {
    () => {
        IterMut!();
        IntoParallelIterator!();
        Iter!();
    };
}

macro_rules! impl_1124 {
    () => {
        deps!();
        impl < 'a , T : Send , E > IntoParallelIterator for & 'a mut Result < T , E > { type Item = & 'a mut T ; type Iter = IterMut < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { IterMut { inner : self . as_mut () . ok () . into_par_iter () , } } }
    };
}

impl_1124!()