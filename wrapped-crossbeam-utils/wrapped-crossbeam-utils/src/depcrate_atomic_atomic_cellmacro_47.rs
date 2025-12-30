// Generated macro for macro_47 (macro)
macro_rules! Depcrate_atomic_atomic_cellmacro_47 {
() => {
// Module: crate::atomic::atomic_cell
// Provides: {"macro_47"}
// Dependencies: {}
# [cfg (not (target_has_atomic = "64"))] impl_arithmetic ! (u64 , fetch_update , "let a = AtomicCell::new(7u64);") ;
};
}
