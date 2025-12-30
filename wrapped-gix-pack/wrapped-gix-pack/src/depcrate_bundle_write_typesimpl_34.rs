// Generated macro for impl_34 (impl)
macro_rules! Depcrate_bundle_write_typesimpl_34 {
() => {
// Module: crate::bundle::write::types
// Provides: {"impl_34"}
// Dependencies: {}
impl io :: Read for LockWriter { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . writer . lock () . get_mut () . read (buf) } }
};
}
