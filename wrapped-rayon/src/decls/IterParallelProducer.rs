macro_rules! deps {
    () => {
        Fuse!();
        Iter!();
    };
}

macro_rules! IterParallelProducer {
    () => {
        deps!();
        struct IterParallelProducer < 'a , Iter > { split_count : AtomicUsize , iter : Mutex < std :: iter :: Fuse < Iter > > , threads_started : & 'a [AtomicBool] , }
    };
}

IterParallelProducer!()