// Generated macro for impl_297 (impl)
macro_rules! Depcrate_indeximpl_297 {
() => {
// Module: crate::index
// Provides: {"impl_297"}
// Dependencies: {}
impl Version { # [doc = " The kind of hash to produce to be compatible to this kind of index"] pub fn hash (& self) -> gix_hash :: Kind { match self { Version :: V1 | Version :: V2 => gix_hash :: Kind :: Sha1 , } } }
};
}
