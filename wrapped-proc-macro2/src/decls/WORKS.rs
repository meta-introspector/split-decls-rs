macro_rules! WORKS {
    () => {
        static WORKS : AtomicUsize = AtomicUsize :: new (0) ;
    };
}

WORKS!();