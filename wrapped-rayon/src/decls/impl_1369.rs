macro_rules! deps {
    () => {
        UnindexedConsumer!();
        ParallelIterator!();
    };
}

macro_rules! impl_1369 {
    () => {
        deps!();
        # [doc = " `Either<L, R>` is a parallel iterator if both `L` and `R` are parallel iterators."] impl < L , R > ParallelIterator for Either < L , R > where L : ParallelIterator , R : ParallelIterator < Item = L :: Item > , { type Item = L :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { match self { Left (iter) => iter . drive_unindexed (consumer) , Right (iter) => iter . drive_unindexed (consumer) , } } fn opt_len (& self) -> Option < usize > { self . as_ref () . either (L :: opt_len , R :: opt_len) } }
    };
}

impl_1369!()