// Generated macro for impl_123 (impl)
macro_rules! Depcrate_asn1impl_123 {
() => {
// Module: crate::asn1
// Provides: {"impl_123"}
// Dependencies: {}
impl Asn1Object { # [doc = " Constructs an ASN.1 Object Identifier from a string representation of the OID."] # [corresponds (OBJ_txt2obj)] # [allow (clippy :: should_implement_trait)] pub fn from_str (txt : & str) -> Result < Asn1Object , ErrorStack > { unsafe { ffi :: init () ; let txt = CString :: new (txt) . unwrap () ; let obj : * mut ffi :: ASN1_OBJECT = cvt_p (ffi :: OBJ_txt2obj (txt . as_ptr () as * const _ , 0)) ? ; Ok (Asn1Object :: from_ptr (obj)) } } # [doc = " Return the OID as an DER encoded array of bytes. This is the ASN.1"] # [doc = " value, not including tag or length."] # [doc = ""] # [doc = " Requires OpenSSL 1.1.1 or newer."] # [corresponds (OBJ_get0_data)] # [cfg (ossl111)] pub fn as_slice (& self) -> & [u8] { unsafe { let len = ffi :: OBJ_length (self . as_ptr ()) ; util :: from_raw_parts (ffi :: OBJ_get0_data (self . as_ptr ()) , len) } } }
};
}
