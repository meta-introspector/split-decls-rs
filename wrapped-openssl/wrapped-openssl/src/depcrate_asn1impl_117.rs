// Generated macro for impl_117 (impl)
macro_rules! Depcrate_asn1impl_117 {
() => {
// Module: crate::asn1
// Provides: {"impl_117"}
// Dependencies: {}
impl Asn1BitStringRef { # [doc = " Returns the Asn1BitString as a slice."] # [corresponds (ASN1_STRING_get0_data)] pub fn as_slice (& self) -> & [u8] { unsafe { util :: from_raw_parts (ASN1_STRING_get0_data (self . as_ptr () as * mut _) , self . len ()) } } # [doc = " Returns the number of bytes in the string."] # [corresponds (ASN1_STRING_length)] pub fn len (& self) -> usize { unsafe { ffi :: ASN1_STRING_length (self . as_ptr () as * const _) as usize } } # [doc = " Determines if the string is empty."] pub fn is_empty (& self) -> bool { self . len () == 0 } }
};
}
