// Generated macro for atomic_ptr (macro)
macro_rules! Depcrate_imp_riscvatomic_ptr {
() => {
// Module: crate::imp::riscv
// Provides: {"atomic_ptr"}
// Dependencies: {}
macro_rules ! atomic_ptr { ($ ([$ ($ generics : tt) *]) ? $ atomic_type : ident , $ value_type : ty $ (as $ cast : ty) ?, $ size : tt) => { atomic_load_store ! ($ ([$ ($ generics) *]) ? $ atomic_type , $ value_type $ (as $ cast) ?, $ size) ; # [cfg (any (test , portable_atomic_force_amo , target_feature = "zaamo" , portable_atomic_target_feature = "zaamo" ,))] impl $ (<$ ($ generics) *>) ? $ atomic_type $ (<$ ($ generics) *>) ? { # [inline] pub (crate) fn swap (& self , val : $ value_type , order : Ordering) -> $ value_type { let dst = self . v . get () ; unsafe { atomic_rmw_amo ! (swap , dst , val $ (as $ cast) ?, order , $ size) $ (as $ cast as $ value_type) ? } } } } ; }
};
}
