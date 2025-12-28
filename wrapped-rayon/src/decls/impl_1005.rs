macro_rules! deps {
    () => {
        WhileSomeConsumer!();
        ParallelIterator!();
        UnindexedConsumer!();
        WhileSome!();
    };
}

macro_rules! impl_1005 {
    () => {
        deps!();
        impl < I , T > ParallelIterator for WhileSome < I > where I : ParallelIterator < Item = Option < T > > , T : Send , { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let full = AtomicBool :: new (false) ; let consumer1 = WhileSomeConsumer { base : consumer , full : & full , } ; self . base . drive_unindexed (consumer1) } }
    };
}

impl_1005!()