// Generated macro for impl_586 (impl)
macro_rules! Depcrate_stabilityimpl_586 {
() => {
// Module: crate::stability
// Provides: {"impl_586"}
// Dependencies: {}
impl UnstableReason { pub fn from_opt_reason (reason : Option < Symbol >) -> Self { match reason { Some (r) => Self :: Some (r) , None => Self :: None , } } pub fn to_opt_reason (& self) -> Option < Symbol > { match self { Self :: None => None , Self :: Default => Some (sym :: unstable_location_reason_default) , Self :: Some (r) => Some (* r) , } } }
};
}
