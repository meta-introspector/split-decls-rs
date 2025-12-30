// Generated macro for impl_1286 (impl)
macro_rules! Depcrate_treeimpl_1286 {
() => {
// Module: crate::tree
// Provides: {"impl_1286"}
// Dependencies: {}
impl < 'repo > std :: fmt :: Debug for Tree < 'repo > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_struct ("Tree") . field ("id" , & self . id ()) . finish () } }
};
}
