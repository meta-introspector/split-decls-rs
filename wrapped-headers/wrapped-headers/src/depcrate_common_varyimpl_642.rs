// Generated macro for impl_642 (impl)
macro_rules! Depcrate_common_varyimpl_642 {
() => {
// Module: crate::common::vary
// Provides: {"impl_642"}
// Dependencies: {}
impl Vary { # [doc = " Create a new `Very: *` header."] pub fn any () -> Vary { Vary (HeaderValue :: from_static ("*") . into ()) } # [doc = " Check if this includes `*`."] pub fn is_any (& self) -> bool { self . 0 . iter () . any (| val | val == "*") } # [doc = " Iterate the header names of this `Vary`."] pub fn iter_strs (& self) -> impl Iterator < Item = & str > { self . 0 . iter () } }
};
}
