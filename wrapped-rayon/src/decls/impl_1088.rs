macro_rules! deps {
    () => {
        Iter!();
        ParallelIterator!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_1088 {
    () => {
        deps!();
        # [doc = " Implemented for ranges of all primitive integer types and `char`."] impl < T > IntoParallelIterator for RangeInclusive < T > where Iter < T > : ParallelIterator , { type Item = < Iter < T > as ParallelIterator > :: Item ; type Iter = Iter < T > ; fn into_par_iter (self) -> Self :: Iter { Iter { range : self } } }
    };
}

impl_1088!()