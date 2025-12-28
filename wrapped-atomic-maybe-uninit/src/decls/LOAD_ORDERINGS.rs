macro_rules! LOAD_ORDERINGS {
    () => {
        pub (crate) const LOAD_ORDERINGS : [Ordering ; 3] = [Ordering :: Relaxed , Ordering :: Acquire , Ordering :: SeqCst] ;
    };
}

LOAD_ORDERINGS!()