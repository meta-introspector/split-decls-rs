macro_rules! deps {
    () => {
        UpdateConsumer!();
        UnindexedConsumer!();
        ParallelIterator!();
        Update!();
    };
}

macro_rules! impl_971 {
    () => {
        deps!();
        impl < I , F > ParallelIterator for Update < I , F > where I : ParallelIterator , F : Fn (& mut I :: Item) + Send + Sync , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = UpdateConsumer :: new (consumer , & self . update_op) ; self . base . drive_unindexed (consumer1) } fn opt_len (& self) -> Option < usize > { self . base . opt_len () } }
    };
}

impl_971!()