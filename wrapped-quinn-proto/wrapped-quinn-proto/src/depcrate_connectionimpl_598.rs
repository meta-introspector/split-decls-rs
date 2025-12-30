// Generated macro for impl_598 (impl)
macro_rules! Depcrate_connectionimpl_598 {
() => {
// Module: crate::connection
// Provides: {"impl_598"}
// Dependencies: {}
impl SideArgs { pub (crate) fn pref_addr_cid (& self) -> Option < ConnectionId > { match * self { Self :: Client { .. } => None , Self :: Server { pref_addr_cid , .. } => pref_addr_cid , } } pub (crate) fn path_validated (& self) -> bool { match * self { Self :: Client { .. } => true , Self :: Server { path_validated , .. } => path_validated , } } pub (crate) fn side (& self) -> Side { match * self { Self :: Client { .. } => Side :: Client , Self :: Server { .. } => Side :: Server , } } }
};
}
