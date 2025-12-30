// Generated macro for impl_103 (impl)
macro_rules! Depcrate_relative_pathimpl_103 {
() => {
// Module: crate::relative_path
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a BString > for & 'a RelativePath { type Error = Error ; fn try_from (value : & 'a BString) -> Result < Self , Self :: Error > { let path = try_from_bstr (value . as_bstr ()) ? ; relative_path_from_value_and_path (value . as_bstr () , & path) } }
};
}
