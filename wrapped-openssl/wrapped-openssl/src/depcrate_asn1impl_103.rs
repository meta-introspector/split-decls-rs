// Generated macro for impl_103 (impl)
macro_rules! Depcrate_asn1impl_103 {
() => {
// Module: crate::asn1
// Provides: {"impl_103"}
// Dependencies: {}
impl Asn1StringRef { # [doc = " Converts the ASN.1 underlying format to UTF8"] # [doc = ""] # [doc = " ASN.1 strings may utilize UTF-16, ASCII, BMP, or UTF8.  This is important to"] # [doc = " consume the string in a meaningful way without knowing the underlying"] # [doc = " format."] # [corresponds (ASN1_STRING_to_UTF8)] pub fn as_utf8 (& self) -> Result < OpensslString , ErrorStack > { unsafe { let mut ptr = ptr :: null_mut () ; let len = ffi :: ASN1_STRING_to_UTF8 (& mut ptr , self . as_ptr ()) ; if len < 0 { return Err (ErrorStack :: get ()) ; } Ok (OpensslString :: from_ptr (ptr as * mut c_char)) } } # [doc = " Return the string as an array of bytes."] # [doc = ""] # [doc = " The bytes do not directly correspond to UTF-8 encoding.  To interact with"] # [doc = " strings in rust, it is preferable to use [`as_utf8`]"] # [doc = ""] # [doc = " [`as_utf8`]: struct.Asn1String.html#method.as_utf8"] # [corresponds (ASN1_STRING_get0_data)] pub fn as_slice (& self) -> & [u8] { unsafe { util :: from_raw_parts (ASN1_STRING_get0_data (self . as_ptr ()) , self . len ()) } } # [doc = " Returns the number of bytes in the string."] # [corresponds (ASN1_STRING_length)] pub fn len (& self) -> usize { unsafe { ffi :: ASN1_STRING_length (self . as_ptr ()) as usize } } # [doc = " Determines if the string is empty."] pub fn is_empty (& self) -> bool { self . len () == 0 } }
};
}
