// Generated macro for impl_100 (impl)
macro_rules! Depcrate_relative_pathimpl_100 {
() => {
// Module: crate::relative_path
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a BStr > for & 'a RelativePath { type Error = Error ; fn try_from (value : & 'a BStr) -> Result < Self , Self :: Error > { let path = try_from_bstr (value) ? ; relative_path_from_value_and_path (value , & path) } }
};
}
