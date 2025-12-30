// Generated macro for impl_551 (impl)
macro_rules! Depcrate_diffimpl_551 {
() => {
// Module: crate::diff
// Provides: {"impl_551"}
// Dependencies: {}
impl std :: fmt :: Debug for DiffStats { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_struct ("DiffStats") . field ("files_changed" , & self . files_changed ()) . field ("insertions" , & self . insertions ()) . field ("deletions" , & self . deletions ()) . finish () } }
};
}
