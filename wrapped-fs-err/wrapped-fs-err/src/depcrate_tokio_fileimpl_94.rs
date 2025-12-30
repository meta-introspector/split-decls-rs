// Generated macro for impl_94 (impl)
macro_rules! Depcrate_tokio_fileimpl_94 {
() => {
// Module: crate::tokio::file
// Provides: {"impl_94"}
// Dependencies: {}
impl From < crate :: File > for File { fn from (f : crate :: File) -> Self { let (f , path) = f . into_parts () ; File :: from_parts (f . into () , path) } }
};
}
