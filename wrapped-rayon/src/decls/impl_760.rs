macro_rules! deps {
    () => {
        IterParallelProducer!();
        IterBridge!();
        UnindexedConsumer!();
        Iter!();
        ParallelIterator!();
    };
}

macro_rules! impl_760 {
    () => {
        deps!();
        impl < Iter > ParallelIterator for IterBridge < Iter > where Iter : Iterator < Item : Send > + Send , { type Item = Iter :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let num_threads = current_num_threads () ; let threads_started : Vec < _ > = (0 .. num_threads) . map (| _ | AtomicBool :: new (false)) . collect () ; bridge_unindexed (& IterParallelProducer { split_count : AtomicUsize :: new (num_threads) , iter : Mutex :: new (self . iter . fuse ()) , threads_started : & threads_started , } , consumer ,) } }
    };
}

impl_760!()