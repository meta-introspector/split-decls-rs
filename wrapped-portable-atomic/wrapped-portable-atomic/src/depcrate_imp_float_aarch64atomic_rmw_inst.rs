// Generated macro for atomic_rmw_inst (macro)
macro_rules! Depcrate_imp_float_aarch64atomic_rmw_inst {
() => {
// Module: crate::imp::float::aarch64
// Provides: {"atomic_rmw_inst"}
// Dependencies: {}
# [cfg (portable_atomic_pre_llvm_20)] macro_rules ! atomic_rmw_inst { ($ op : ident , $ order : ident) => { atomic_rmw_inst ! ($ op , $ order , write = $ order) } ; ($ op : ident , $ order : ident , write = $ write : ident) => { match $ order { Ordering :: Relaxed => $ op ! ("2" , "") , Ordering :: Acquire => $ op ! ("a" , "") , Ordering :: Release => $ op ! ("6" , "") , Ordering :: AcqRel => $ op ! ("e" , "") , # [cfg (target_env = "msvc")] Ordering :: SeqCst if $ write == Ordering :: SeqCst => $ op ! ("e" , "dmb ish") , Ordering :: SeqCst => $ op ! ("e" , "") , _ => unreachable ! () , } } ; }
};
}
