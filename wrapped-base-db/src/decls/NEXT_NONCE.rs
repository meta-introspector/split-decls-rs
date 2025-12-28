macro_rules! NEXT_NONCE {
    () => {
        static NEXT_NONCE : AtomicUsize = AtomicUsize :: new (0) ;
    };
}

NEXT_NONCE!();