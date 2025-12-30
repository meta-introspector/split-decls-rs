// Generated macro for COMPARE_EXCHANGE_ORDERINGS (const)
macro_rules! Depcrate_tests_helperCOMPARE_EXCHANGE_ORDERINGS {
() => {
// Module: crate::tests::helper
// Provides: {"COMPARE_EXCHANGE_ORDERINGS"}
// Dependencies: {}
pub (crate) const COMPARE_EXCHANGE_ORDERINGS : [(Ordering , Ordering) ; 15] = [(Ordering :: Relaxed , Ordering :: Relaxed) , (Ordering :: Relaxed , Ordering :: Acquire) , (Ordering :: Relaxed , Ordering :: SeqCst) , (Ordering :: Acquire , Ordering :: Relaxed) , (Ordering :: Acquire , Ordering :: Acquire) , (Ordering :: Acquire , Ordering :: SeqCst) , (Ordering :: Release , Ordering :: Relaxed) , (Ordering :: Release , Ordering :: Acquire) , (Ordering :: Release , Ordering :: SeqCst) , (Ordering :: AcqRel , Ordering :: Relaxed) , (Ordering :: AcqRel , Ordering :: Acquire) , (Ordering :: AcqRel , Ordering :: SeqCst) , (Ordering :: SeqCst , Ordering :: Relaxed) , (Ordering :: SeqCst , Ordering :: Acquire) , (Ordering :: SeqCst , Ordering :: SeqCst) ,] ;
};
}
