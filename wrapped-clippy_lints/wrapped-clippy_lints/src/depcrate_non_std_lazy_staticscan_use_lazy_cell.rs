// Generated macro for can_use_lazy_cell (function)
macro_rules! Depcrate_non_std_lazy_staticscan_use_lazy_cell {
() => {
// Module: crate::non_std_lazy_statics
// Provides: {"can_use_lazy_cell"}
// Dependencies: {}
fn can_use_lazy_cell (cx : & LateContext < '_ > , msrv : Msrv) -> bool { msrv . meets (cx , msrvs :: LAZY_CELL) && ! is_no_std_crate (cx) }
};
}
