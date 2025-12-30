// Generated macro for impl_824 (impl)
macro_rules! Depcrate_machineimpl_824 {
() => {
// Module: crate::machine
// Provides: {"impl_824"}
// Dependencies: {}
impl fmt :: Debug for ProvenanceExtra { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ProvenanceExtra :: Concrete (pid) => write ! (f , "{pid:?}") , ProvenanceExtra :: Wildcard => write ! (f , "<wildcard>") , } } }
};
}
