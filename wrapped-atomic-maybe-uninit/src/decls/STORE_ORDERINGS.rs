macro_rules! STORE_ORDERINGS {
    () => {
        pub (crate) const STORE_ORDERINGS : [Ordering ; 3] = [Ordering :: Relaxed , Ordering :: Release , Ordering :: SeqCst] ;
    };
}

STORE_ORDERINGS!();