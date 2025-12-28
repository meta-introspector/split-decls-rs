macro_rules! deps {
    () => {
        ParSymmetricDifference!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < 'a , T , S , A > ParallelIterator for ParSymmetricDifference < 'a , T , S , A > where T : Eq + Hash + Sync , S : BuildHasher + Sync , A : Allocator + Sync , { type Item = & 'a T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . a . par_difference (self . b) . chain (self . b . par_difference (self . a)) . drive_unindexed (consumer) } }
    };
}

impl_189!()