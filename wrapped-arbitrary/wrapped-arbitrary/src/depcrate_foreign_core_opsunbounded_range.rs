// Generated macro for unbounded_range (function)
macro_rules! Depcrate_foreign_core_opsunbounded_range {
() => {
// Module: crate::foreign::core::ops
// Provides: {"unbounded_range"}
// Dependencies: {}
pub (crate) fn unbounded_range < CB , I , R > (bound : I , cb : CB) -> R where CB : Fn (I) -> R , R : RangeBounds < I > , { cb (bound) }
};
}
