// Generated macro for impl_35 (impl)
macro_rules! Depcrate_bundle_write_typesimpl_35 {
() => {
// Module: crate::bundle::write::types
// Provides: {"impl_35"}
// Dependencies: {}
impl io :: Seek for LockWriter { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { self . writer . lock () . seek (pos) } }
};
}
