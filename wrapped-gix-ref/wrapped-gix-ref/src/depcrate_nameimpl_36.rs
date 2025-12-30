// Generated macro for impl_36 (impl)
macro_rules! Depcrate_nameimpl_36 {
() => {
// Module: crate::name
// Provides: {"impl_36"}
// Dependencies: {}
impl PartialNameRef { # [doc = " Convert this name into the relative path possibly identifying the reference location."] # [doc = " Note that it may be only a partial path though."] pub fn to_partial_path (& self) -> & Path { gix_path :: from_byte_slice (self . 0 . as_bstr ()) } # [doc = " Provide the name as binary string which is known to be a valid partial ref name."] pub fn as_bstr (& self) -> & BStr { & self . 0 } }
};
}
