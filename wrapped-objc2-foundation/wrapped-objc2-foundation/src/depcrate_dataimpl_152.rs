// Generated macro for impl_152 (impl)
macro_rules! Depcrate_dataimpl_152 {
() => {
// Module: crate::data
// Provides: {"impl_152"}
// Dependencies: {}
impl NSData { pub fn with_bytes (bytes : & [u8]) -> Retained < Self > { let bytes_ptr = bytes . as_ptr () as * mut c_void ; unsafe { Self :: initWithBytes_length (Self :: alloc () , bytes_ptr , bytes . len ()) } } # [cfg (feature = "block2")] # [cfg (feature = "alloc")] pub fn from_vec (bytes : Vec < u8 >) -> Retained < Self > { unsafe { with_vec (Self :: alloc () , bytes) } } }
};
}
