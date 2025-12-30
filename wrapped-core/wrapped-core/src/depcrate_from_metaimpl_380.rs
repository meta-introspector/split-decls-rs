// Generated macro for impl_380 (impl)
macro_rules! Depcrate_from_metaimpl_380 {
() => {
// Module: crate::from_meta
// Provides: {"impl_380"}
// Dependencies: {}
impl FromMeta for char { # [allow (clippy :: wrong_self_convention)] fn from_char (value : char) -> Result < Self > { Ok (value) } fn from_string (s : & str) -> Result < Self > { let mut chars = s . chars () ; let char1 = chars . next () ; let char2 = chars . next () ; if let (Some (char) , None) = (char1 , char2) { Ok (char) } else { Err (Error :: unexpected_type ("string")) } } }
};
}
