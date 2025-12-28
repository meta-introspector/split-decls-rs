macro_rules! deps {
    () => {
        ParallelIterator!();
        IntoParallelIterator!();
        Iter!();
    };
}

macro_rules! impl_1030 {
    () => {
        deps!();
        impl < T : ParallelIterator > IntoParallelIterator for T { type Iter = T ; type Item = T :: Item ; fn into_par_iter (self) -> T { self } }
    };
}

impl_1030!()