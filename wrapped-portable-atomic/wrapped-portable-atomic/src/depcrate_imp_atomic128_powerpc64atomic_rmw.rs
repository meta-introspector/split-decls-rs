// Generated macro for atomic_rmw (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64atomic_rmw {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"atomic_rmw"}
// Dependencies: {}
macro_rules ! atomic_rmw { ($ op : ident , $ order : ident) => { match $ order { Ordering :: Relaxed => $ op ! ("" , "") , Ordering :: Acquire => $ op ! ("isync" , "") , Ordering :: Release => $ op ! ("" , "lwsync") , Ordering :: AcqRel => $ op ! ("isync" , "lwsync") , Ordering :: SeqCst => $ op ! ("isync" , "sync") , _ => unreachable ! () , } } ; }
};
}
