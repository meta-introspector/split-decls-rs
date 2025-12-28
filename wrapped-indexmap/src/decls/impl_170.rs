macro_rules! deps {
    () => {
        ParIntersection!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < 'a , T , S1 , S2 > ParallelIterator for ParIntersection < 'a , T , S1 , S2 > where T : Hash + Eq + Sync , S1 : BuildHasher + Sync , S2 : BuildHasher + Sync , { type Item = & 'a T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let Self { set1 , set2 } = self ; set1 . par_iter () . filter (move | & item | set2 . contains (item)) . drive_unindexed (consumer) } }
    };
}

impl_170!();