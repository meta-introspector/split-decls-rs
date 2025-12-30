// Generated macro for impl_331 (impl)
macro_rules! Depcrate_extensionsimpl_331 {
() => {
// Module: crate::extensions
// Provides: {"impl_331"}
// Dependencies: {}
impl ExtensionType { # [allow (dead_code)] pub (crate) const fn try_from_byte_slice (key : & [u8]) -> Result < Self , ParseError > { if let [b] = key { Self :: try_from_byte (* b) } else { Err (ParseError :: InvalidExtension) } } pub (crate) const fn try_from_byte (key : u8) -> Result < Self , ParseError > { let key = key . to_ascii_lowercase () ; match key as char { UNICODE_EXT_CHAR => Ok (Self :: Unicode) , TRANSFORM_EXT_CHAR => Ok (Self :: Transform) , PRIVATE_EXT_CHAR => Ok (Self :: Private) , 'a' ..= 'z' => Ok (Self :: Other (key)) , _ => Err (ParseError :: InvalidExtension) , } } pub (crate) const fn try_from_utf8 (code_units : & [u8]) -> Result < Self , ParseError > { let & [first] = code_units else { return Err (ParseError :: InvalidExtension) ; } ; Self :: try_from_byte (first) } }
};
}
