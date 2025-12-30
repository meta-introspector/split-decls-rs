// Generated macro for impl_211 (impl)
macro_rules! Depcrate_blobimpl_211 {
() => {
// Module: crate::blob
// Provides: {"impl_211"}
// Dependencies: {}
impl crate :: WriteTo for Blob { # [doc = " Write the blobs data to `out` verbatim."] fn write_to (& self , out : & mut dyn io :: Write) -> io :: Result < () > { self . to_ref () . write_to (out) } fn kind (& self) -> Kind { Kind :: Blob } fn size (& self) -> u64 { self . to_ref () . size () } }
};
}
