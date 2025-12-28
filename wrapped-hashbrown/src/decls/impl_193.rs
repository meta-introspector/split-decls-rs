macro_rules! deps {
    () => {
        ParUnion!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < 'a , T , S , A > ParallelIterator for ParUnion < 'a , T , S , A > where T : Eq + Hash + Sync , S : BuildHasher + Sync , A : Allocator + Sync , { type Item = & 'a T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let (smaller , larger) = if self . a . len () <= self . b . len () { (self . a , self . b) } else { (self . b , self . a) } ; larger . into_par_iter () . chain (smaller . par_difference (larger)) . drive_unindexed (consumer) } }
    };
}

impl_193!();