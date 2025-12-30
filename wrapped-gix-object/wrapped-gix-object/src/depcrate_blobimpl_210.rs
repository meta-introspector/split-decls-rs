// Generated macro for impl_210 (impl)
macro_rules! Depcrate_blobimpl_210 {
() => {
// Module: crate::blob
// Provides: {"impl_210"}
// Dependencies: {}
impl crate :: WriteTo for BlobRef < '_ > { # [doc = " Write the blobs data to `out` verbatim."] fn write_to (& self , out : & mut dyn io :: Write) -> io :: Result < () > { out . write_all (self . data) } fn kind (& self) -> Kind { Kind :: Blob } fn size (& self) -> u64 { self . data . len () as u64 } }
};
}
