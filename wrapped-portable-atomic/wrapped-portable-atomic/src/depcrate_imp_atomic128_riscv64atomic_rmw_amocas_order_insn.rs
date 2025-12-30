// Generated macro for atomic_rmw_amocas_order_insn (macro)
macro_rules! Depcrate_imp_atomic128_riscv64atomic_rmw_amocas_order_insn {
() => {
// Module: crate::imp::atomic128::riscv64
// Provides: {"atomic_rmw_amocas_order_insn"}
// Dependencies: {}
# [cfg (portable_atomic_pre_llvm_20)] macro_rules ! atomic_rmw_amocas_order_insn { ($ op : ident , $ order : ident) => { atomic_rmw_amocas_order_insn ! ($ op , $ order , failure = $ order) } ; ($ op : ident , $ order : ident , failure = $ failure : ident) => { match $ order { Ordering :: Relaxed => $ op ! ("" , "8") , Ordering :: Acquire => $ op ! ("" , "c") , Ordering :: Release => $ op ! ("" , "a") , Ordering :: AcqRel => $ op ! ("" , "e") , Ordering :: SeqCst if $ failure == Ordering :: SeqCst => $ op ! ("fence rw,rw" , "e") , Ordering :: SeqCst => $ op ! ("" , "e") , _ => unreachable ! () , } } ; }
};
}
