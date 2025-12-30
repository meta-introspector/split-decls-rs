// Generated macro for impl_990 (impl)
macro_rules! Depcrate_machineimpl_990 {
() => {
// Module: crate::machine
// Provides: {"impl_990"}
// Dependencies: {}
impl ProvenanceExtra { pub fn and_then < T > (self , f : impl FnOnce (BorTag) -> Option < T >) -> Option < T > { match self { ProvenanceExtra :: Concrete (pid) => f (pid) , ProvenanceExtra :: Wildcard => None , } } }
};
}
