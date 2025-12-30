// Generated macro for impl_9743 (impl)
macro_rules! Depcrate_suspicious_operation_groupingsimpl_9743 {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"impl_9743"}
// Dependencies: {}
impl IdentDifference { # [doc = " Returns true if learning about more differences will not change the value"] # [doc = " of this `IdentDifference`, and false otherwise."] fn is_complete (& self) -> bool { match self { Self :: NoDifference | Self :: Single (_) | Self :: Double (_ , _) => false , Self :: Multiple | Self :: NonIdent => true , } } }
};
}
