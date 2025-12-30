// Generated macro for impl_90 (impl)
macro_rules! Depcrate_walkimpl_90 {
() => {
// Module: crate::walk
// Provides: {"impl_90"}
// Dependencies: {}
impl std :: fmt :: Debug for DirEntryRaw { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("DirEntryRaw") . field ("path" , & self . path) . field ("follow_link" , & self . follow_link) . field ("depth" , & self . depth) . finish () } }
};
}
