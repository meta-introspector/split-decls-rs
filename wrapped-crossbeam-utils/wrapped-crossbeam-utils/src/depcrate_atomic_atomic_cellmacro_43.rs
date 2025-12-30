// Generated macro for macro_43 (macro)
macro_rules! Depcrate_atomic_atomic_cellmacro_43 {
() => {
// Module: crate::atomic::atomic_cell
// Provides: {"macro_43"}
// Dependencies: {}
# [cfg (not (target_has_atomic = "32"))] impl_arithmetic ! (u32 , fetch_update , "let a = AtomicCell::new(7u32);") ;
};
}
