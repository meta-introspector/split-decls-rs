// Generated macro for impl_155 (impl)
macro_rules! Depcrate_dataimpl_155 {
() => {
// Module: crate::data
// Provides: {"impl_155"}
// Dependencies: {}
impl NSMutableData { # [doc = " A mutable view of the bytes in the data."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " No methods on the `NSMutableData` may be called while the returned"] # [doc = " slice is alive."] # [doc (alias = "mutableBytes")] # [allow (clippy :: mut_from_ref)] pub unsafe fn as_mut_bytes_unchecked (& self) -> & mut [u8] { let ptr = self . mutable_bytes_raw () ; if ! ptr . is_null () { let ptr : * mut u8 = ptr . cast () ; unsafe { slice :: from_raw_parts_mut (ptr , self . len ()) } } else { & mut [] } } # [doc (alias = "appendBytes:length:")] pub fn extend_from_slice (& self , bytes : & [u8]) { let bytes_ptr : NonNull < c_void > = NonNull :: new (bytes . as_ptr () as * mut u8) . unwrap () . cast () ; unsafe { self . appendBytes_length (bytes_ptr , bytes . len ()) } } pub fn push (& self , byte : u8) { self . extend_from_slice (& [byte]) ; } # [doc (alias = "replaceBytesInRange:withBytes:length:")] # [cfg (feature = "NSRange")] pub fn replace_range (& self , range : Range < usize > , bytes : & [u8]) { let ptr = bytes . as_ptr () . cast () ; unsafe { self . replaceBytesInRange_withBytes_length (range . into () , ptr , bytes . len ()) } } # [cfg (feature = "NSRange")] pub fn set_bytes (& self , bytes : & [u8]) { let len = self . len () ; self . replace_range (0 .. len , bytes) ; } }
};
}
