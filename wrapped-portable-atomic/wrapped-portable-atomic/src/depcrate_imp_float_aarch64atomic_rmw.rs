// Generated macro for atomic_rmw (macro)
macro_rules! Depcrate_imp_float_aarch64atomic_rmw {
() => {
// Module: crate::imp::float::aarch64
// Provides: {"atomic_rmw"}
// Dependencies: {}
# [cfg (not (portable_atomic_pre_llvm_20))] macro_rules ! atomic_rmw { ($ op : ident , $ order : ident) => { atomic_rmw ! ($ op , $ order , write = $ order) } ; ($ op : ident , $ order : ident , write = $ write : ident) => { match $ order { Ordering :: Relaxed => $ op ! ("" , "" , "") , Ordering :: Acquire => $ op ! ("a" , "" , "") , Ordering :: Release => $ op ! ("" , "l" , "") , Ordering :: AcqRel => $ op ! ("a" , "l" , "") , # [cfg (target_env = "msvc")] Ordering :: SeqCst if $ write == Ordering :: SeqCst => $ op ! ("a" , "l" , "dmb ish") , Ordering :: SeqCst => $ op ! ("a" , "l" , "") , _ => unreachable ! () , } } ; }
};
}
