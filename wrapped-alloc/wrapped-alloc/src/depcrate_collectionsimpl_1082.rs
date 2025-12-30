// Generated macro for impl_1082 (impl)
macro_rules! Depcrate_collectionsimpl_1082 {
() => {
// Module: crate::collections
// Provides: {"impl_1082"}
// Dependencies: {}
# [cfg (not (test))] impl TryReserveError { # [doc = " Details about the allocation that caused the error"] # [inline] # [must_use] # [unstable (feature = "try_reserve_kind" , reason = "Uncertain how much info should be exposed" , issue = "48043")] pub fn kind (& self) -> TryReserveErrorKind { self . kind . clone () } }
};
}
