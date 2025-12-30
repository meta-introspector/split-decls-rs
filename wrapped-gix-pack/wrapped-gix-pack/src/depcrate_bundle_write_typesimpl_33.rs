// Generated macro for impl_33 (impl)
macro_rules! Depcrate_bundle_write_typesimpl_33 {
() => {
// Module: crate::bundle::write::types
// Provides: {"impl_33"}
// Dependencies: {}
impl io :: Write for LockWriter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . writer . lock () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . writer . lock () . flush () } }
};
}
