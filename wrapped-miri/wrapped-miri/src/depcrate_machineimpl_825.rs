// Generated macro for impl_825 (impl)
macro_rules! Depcrate_machineimpl_825 {
() => {
// Module: crate::machine
// Provides: {"impl_825"}
// Dependencies: {}
impl ProvenanceExtra { pub fn and_then < T > (self , f : impl FnOnce (BorTag) -> Option < T >) -> Option < T > { match self { ProvenanceExtra :: Concrete (pid) => f (pid) , ProvenanceExtra :: Wildcard => None , } } }
};
}
