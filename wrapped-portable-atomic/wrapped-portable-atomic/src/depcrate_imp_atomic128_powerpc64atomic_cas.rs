// Generated macro for atomic_cas (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64atomic_cas {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"atomic_cas"}
// Dependencies: {}
macro_rules ! atomic_cas { ($ op : ident , $ success : ident , $ failure : ident) => { if $ failure == Ordering :: Relaxed { match $ success { Ordering :: Relaxed => $ op ! ("" , "" , "") , Ordering :: Acquire => $ op ! ("" , "isync" , "") , Ordering :: Release => $ op ! ("" , "" , "lwsync") , Ordering :: AcqRel => $ op ! ("" , "isync" , "lwsync") , Ordering :: SeqCst => $ op ! ("" , "isync" , "sync") , _ => unreachable ! () , } } else { let order = crate :: utils :: upgrade_success_ordering ($ success , $ failure) ; match order { Ordering :: Acquire => $ op ! ("isync" , "" , "") , Ordering :: AcqRel => $ op ! ("isync" , "" , "lwsync") , Ordering :: SeqCst => $ op ! ("isync" , "" , "sync") , _ => unreachable ! () , } } } ; }
};
}
