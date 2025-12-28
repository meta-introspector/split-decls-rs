macro_rules! SWAP_ORDERINGS {
    () => {
        pub (crate) const SWAP_ORDERINGS : [Ordering ; 5] = [Ordering :: Relaxed , Ordering :: Release , Ordering :: Acquire , Ordering :: AcqRel , Ordering :: SeqCst] ;
    };
}

SWAP_ORDERINGS!();