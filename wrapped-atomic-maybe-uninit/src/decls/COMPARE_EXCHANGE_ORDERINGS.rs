macro_rules! COMPARE_EXCHANGE_ORDERINGS {
    () => {
        pub (crate) const COMPARE_EXCHANGE_ORDERINGS : [(Ordering , Ordering) ; 15] = [(Ordering :: Relaxed , Ordering :: Relaxed) , (Ordering :: Relaxed , Ordering :: Acquire) , (Ordering :: Relaxed , Ordering :: SeqCst) , (Ordering :: Acquire , Ordering :: Relaxed) , (Ordering :: Acquire , Ordering :: Acquire) , (Ordering :: Acquire , Ordering :: SeqCst) , (Ordering :: Release , Ordering :: Relaxed) , (Ordering :: Release , Ordering :: Acquire) , (Ordering :: Release , Ordering :: SeqCst) , (Ordering :: AcqRel , Ordering :: Relaxed) , (Ordering :: AcqRel , Ordering :: Acquire) , (Ordering :: AcqRel , Ordering :: SeqCst) , (Ordering :: SeqCst , Ordering :: Relaxed) , (Ordering :: SeqCst , Ordering :: Acquire) , (Ordering :: SeqCst , Ordering :: SeqCst) ,] ;
    };
}

COMPARE_EXCHANGE_ORDERINGS!();