// Generated macro for other_1085 (other)
macro_rules! Depcrate_collectionsother_1085 {
() => {
// Module: crate::collections
// Provides: {"other_1085"}
// Dependencies: {}
# [unstable (feature = "try_reserve_kind" , reason = "Uncertain how much info should be exposed" , issue = "48043")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] # [cfg (not (test))] impl const From < TryReserveErrorKind > for TryReserveError { # [inline] fn from (kind : TryReserveErrorKind) -> Self { Self { kind } } }
};
}
