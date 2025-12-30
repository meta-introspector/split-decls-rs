// Generated macro for impl_56 (impl)
macro_rules! Depcrate_into_urlimpl_56 {
() => {
// Module: crate::into_url
// Provides: {"impl_56"}
// Dependencies: {}
impl IntoUrlSealed for Url { fn into_url (self) -> crate :: Result < Url > { # [cfg (target_arch = "wasm32")] if self . scheme () == "blob" && self . path () . starts_with ("http") && self . as_str () [5 ..] . into_url () . is_ok () { return Ok (self) ; } if self . has_host () { Ok (self) } else { Err (crate :: error :: url_bad_scheme (self)) } } fn as_str (& self) -> & str { self . as_ref () } }
};
}
