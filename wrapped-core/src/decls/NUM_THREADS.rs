macro_rules! NUM_THREADS {
    () => {
        static NUM_THREADS : AtomicUsize = AtomicUsize :: new (0) ;
    };
}

NUM_THREADS!();