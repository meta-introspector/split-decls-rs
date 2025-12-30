// Generated macro for macro_48 (macro)
macro_rules! Depcrate_atomic_atomic_cellmacro_48 {
() => {
// Module: crate::atomic::atomic_cell
// Provides: {"macro_48"}
// Dependencies: {}
# [cfg (not (target_has_atomic = "64"))] impl_arithmetic ! (i64 , fetch_update , "let a = AtomicCell::new(7i64);") ;
};
}
