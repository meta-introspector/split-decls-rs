// Generated macro for impl_128 (impl)
macro_rules! Depcrateimpl_128 {
() => {
// Module: crate
// Provides: {"impl_128"}
// Dependencies: {}
impl SharedLibraryId { # [doc = " Returns the raw bytes of the shared library ID."] pub fn as_bytes (& self) -> & [u8] { match * self { SharedLibraryId :: Uuid (ref bytes) => & * bytes , SharedLibraryId :: GnuBuildId (ref bytes) => bytes , SharedLibraryId :: PeSignature (_ , _) => & [] [..] , SharedLibraryId :: PdbSignature (ref bytes , _) => & * bytes , } } }
};
}
