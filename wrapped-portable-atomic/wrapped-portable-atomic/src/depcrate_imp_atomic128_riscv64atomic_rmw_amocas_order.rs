// Generated macro for atomic_rmw_amocas_order (macro)
macro_rules! Depcrate_imp_atomic128_riscv64atomic_rmw_amocas_order {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"atomic_rmw_amocas_order"}
// Dependencies: {}
# [cfg (not (portable_atomic_pre_llvm_20))] macro_rules ! atomic_rmw_amocas_order { ($ op : ident , $ order : ident) => { atomic_rmw_amocas_order ! ($ op , $ order , failure = $ order) } ; ($ op : ident , $ order : ident , failure = $ failure : ident) => { match $ order { Ordering :: Relaxed => $ op ! ("" , "") , Ordering :: Acquire => $ op ! ("" , ".aq") , Ordering :: Release => $ op ! ("" , ".rl") , Ordering :: AcqRel => $ op ! ("" , ".aqrl") , Ordering :: SeqCst if $ failure == Ordering :: SeqCst => $ op ! ("fence rw,rw" , ".aqrl") , Ordering :: SeqCst => $ op ! ("" , ".aqrl") , _ => unreachable ! () , } } ; }
};
}
