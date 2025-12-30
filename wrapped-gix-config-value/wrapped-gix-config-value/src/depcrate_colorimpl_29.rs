// Generated macro for impl_29 (impl)
macro_rules! Depcrate_colorimpl_29 {
() => {
// Module: crate::color
// Provides: {"impl_29"}
// Dependencies: {}
impl TryFrom < & BStr > for Name { type Error = Error ; fn try_from (s : & BStr) -> Result < Self , Self :: Error > { Self :: from_str (std :: str :: from_utf8 (s) . map_err (| err | color_err (s) . with_err (err)) ?) } }
};
}
