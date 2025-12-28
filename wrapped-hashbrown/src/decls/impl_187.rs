macro_rules! deps {
    () => {
        ParDifference!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < 'a , T , S , A > ParallelIterator for ParDifference < 'a , T , S , A > where T : Eq + Hash + Sync , S : BuildHasher + Sync , A : Allocator + Sync , { type Item = & 'a T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . a . into_par_iter () . filter (| & x | ! self . b . contains (x)) . drive_unindexed (consumer) } }
    };
}

impl_187!();