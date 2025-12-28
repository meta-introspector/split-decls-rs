macro_rules! CALL_LIMIT {
    () => {
        static CALL_LIMIT : AtomicUsize = AtomicUsize :: new (0) ;
    };
}

CALL_LIMIT!();