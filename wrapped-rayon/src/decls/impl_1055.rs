macro_rules! deps {
    () => {
        ParallelIterator!();
        Iter!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_1055 {
    () => {
        deps!();
        # [doc = " Implemented for ranges of all primitive integer types and `char`."] impl < T > IntoParallelIterator for Range < T > where Iter < T > : ParallelIterator , { type Item = < Iter < T > as ParallelIterator > :: Item ; type Iter = Iter < T > ; fn into_par_iter (self) -> Self :: Iter { Iter { range : self } } }
    };
}

impl_1055!();