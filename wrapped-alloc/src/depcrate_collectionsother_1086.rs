// Generated macro for other_1086 (other)
macro_rules! Depcrate_collectionsother_1086 {
() => {
// Module: crate::collections
// Provides: {"other_1086"}
// Dependencies: {}
# [unstable (feature = "try_reserve_kind" , reason = "new API" , issue = "48043")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] # [cfg (not (test))] impl const From < LayoutError > for TryReserveErrorKind { # [doc = " Always evaluates to [`TryReserveErrorKind::CapacityOverflow`]."] # [inline] fn from (_ : LayoutError) -> Self { TryReserveErrorKind :: CapacityOverflow } }
};
}
