// Generated macro for impl_989 (impl)
macro_rules! Depcrate_machineimpl_989 {
() => {
// Module: crate::machine
// Provides: {"impl_989"}
// Dependencies: {}
impl fmt :: Debug for ProvenanceExtra { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ProvenanceExtra :: Concrete (pid) => write ! (f , "{pid:?}") , ProvenanceExtra :: Wildcard => write ! (f , "<wildcard>") , } } }
};
}
