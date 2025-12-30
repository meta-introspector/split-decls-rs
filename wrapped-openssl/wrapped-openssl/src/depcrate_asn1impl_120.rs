// Generated macro for impl_120 (impl)
macro_rules! Depcrate_asn1impl_120 {
() => {
// Module: crate::asn1
// Provides: {"impl_120"}
// Dependencies: {}
impl Asn1OctetStringRef { # [doc = " Returns the octet string as an array of bytes."] # [corresponds (ASN1_STRING_get0_data)] pub fn as_slice (& self) -> & [u8] { unsafe { util :: from_raw_parts (ASN1_STRING_get0_data (self . as_ptr () . cast ()) , self . len ()) } } # [doc = " Returns the number of bytes in the octet string."] # [corresponds (ASN1_STRING_length)] pub fn len (& self) -> usize { unsafe { ffi :: ASN1_STRING_length (self . as_ptr () . cast ()) as usize } } # [doc = " Determines if the string is empty."] pub fn is_empty (& self) -> bool { self . len () == 0 } }
};
}
