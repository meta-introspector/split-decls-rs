// Generated macro for impl_23 (impl)
macro_rules! Depcrate_fieldsimpl_23 {
() => {
// Module: crate::fields
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'a > Field < 'a > { # [doc = " Create a new [`Field`], validating the provided characters are in the allowed range."] pub fn new (s : & 'a str) -> Result < Self > { let field = Field (s) ; field . validate () ? ; Ok (field) } # [doc = " Borrow the field's contents as a `str`."] pub fn as_str (self) -> & 'a str { self . 0 } # [doc = " Decode Base64 into the provided output buffer."] # [cfg (feature = "base64")] pub fn decode_base64_into (self , base64_variant : Base64 , out : & mut [u8]) -> Result < & [u8] > { base64_variant . decode (self . 0 , out) . map_err (| _ | Error { }) } # [doc = " Decode this field as the provided Base64 variant."] # [cfg (all (feature = "alloc" , feature = "base64"))] pub fn decode_base64 (self , base64_variant : Base64) -> Result < Vec < u8 > > { base64_variant . decode_vec (self . 0) . map_err (| _ | Error { }) } # [doc = " Validate a field in the password hash is well-formed."] pub (crate) fn validate (self) -> Result < () > { if self . 0 . is_empty () { return Err (Error { }) ; } for c in self . 0 . chars () { match c { 'A' ..= 'Z' | 'a' ..= 'z' | '0' ..= '9' | '.' | '/' | '+' | '=' | ',' | '-' => () , _ => return Err (Error { }) , } } Ok (()) } }
};
}
