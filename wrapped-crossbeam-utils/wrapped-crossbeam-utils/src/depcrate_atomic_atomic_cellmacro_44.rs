// Generated macro for macro_44 (macro)
macro_rules! Depcrate_atomic_atomic_cellmacro_44 {
() => {
// Module: crate::atomic::atomic_cell
// Provides: {"macro_44"}
// Dependencies: {}
# [cfg (not (target_has_atomic = "32"))] impl_arithmetic ! (i32 , fetch_update , "let a = AtomicCell::new(7i32);") ;
};
}
