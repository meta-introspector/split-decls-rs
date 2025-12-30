// Generated macro for atomic_rmw_amo (macro)
macro_rules! Depcrate_imp_riscvatomic_rmw_amo {
() => {
// Module: crate::imp::riscv
// Provides: {"atomic_rmw_amo"}
// Dependencies: {}
# [cfg (any (test , portable_atomic_force_amo , target_feature = "zaamo" , portable_atomic_target_feature = "zaamo" ,))] macro_rules ! atomic_rmw_amo { ($ op : ident , $ dst : ident , $ val : ident $ (as $ cast : ty) ?, $ order : ident , $ size : tt) => { { let out $ (: $ cast) ?; macro_rules ! op { ($ asm_order : tt) => { asm ! (".option push" , concat ! (".option arch, " , atomic_rmw_amo_ext ! ($ size)) , concat ! ("amo" , stringify ! ($ op) , "." , $ size , $ asm_order , " {out}, {val}, 0({dst})") , ".option pop" , dst = in (reg) ptr_reg ! ($ dst) , val = in (reg) $ val $ (as $ cast) ?, out = lateout (reg) out , options (nostack , preserves_flags) ,) } ; } match $ order { Ordering :: Relaxed => op ! ("") , Ordering :: Acquire => op ! (".aq") , Ordering :: Release => op ! (".rl") , Ordering :: AcqRel | Ordering :: SeqCst => op ! (".aqrl") , _ => unreachable ! () , } out } } ; }
};
}
