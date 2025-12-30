// Generated macro for impl_85 (impl)
macro_rules! Depcrate_deimpl_85 {
() => {
// Module: crate::de
// Provides: {"impl_85"}
// Dependencies: {}
impl < 'de > Deserializer < 'de > { # [allow (clippy :: should_implement_trait)] pub fn from_str (input : & 'de str) -> SpannedResult < Self > { Self :: from_str_with_options (input , & Options :: default ()) } pub fn from_bytes (input : & 'de [u8]) -> SpannedResult < Self > { Self :: from_bytes_with_options (input , & Options :: default ()) } pub fn from_str_with_options (input : & 'de str , options : & Options) -> SpannedResult < Self > { let mut deserializer = Deserializer { parser : Parser :: new (input) ? , newtype_variant : false , serde_content_newtype : false , last_identifier : None , recursion_limit : options . recursion_limit , } ; deserializer . parser . exts |= options . default_extensions ; Ok (deserializer) } # [allow (clippy :: missing_panics_doc)] pub fn from_bytes_with_options (input : & 'de [u8] , options : & Options) -> SpannedResult < Self > { let err = match str :: from_utf8 (input) { Ok (input) => return Self :: from_str_with_options (input , options) , Err (err) => err , } ; # [allow (clippy :: expect_used)] let valid_input = str :: from_utf8 (& input [.. err . valid_up_to ()]) . expect ("source is valid up to error") ; Err (SpannedError { code : err . into () , span : Span { start : Position { line : 1 , col : 1 } , end : Position :: from_src_end (valid_input) , } , }) } # [must_use] pub fn remainder (& self) -> & 'de str { self . parser . src () } # [must_use] pub fn span_error (& self , code : Error) -> SpannedError { self . parser . span_error (code) } # [must_use] pub fn extensions (& self) -> Extensions { self . parser . exts } }
};
}
