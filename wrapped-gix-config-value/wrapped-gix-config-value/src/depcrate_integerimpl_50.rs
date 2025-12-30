// Generated macro for impl_50 (impl)
macro_rules! Depcrate_integerimpl_50 {
() => {
// Module: crate::integer
// Provides: {"impl_50"}
// Dependencies: {}
impl TryFrom < & BStr > for Suffix { type Error = () ; fn try_from (s : & BStr) -> Result < Self , Self :: Error > { Self :: from_str (std :: str :: from_utf8 (s) . map_err (| _ | ()) ?) } }
};
}
