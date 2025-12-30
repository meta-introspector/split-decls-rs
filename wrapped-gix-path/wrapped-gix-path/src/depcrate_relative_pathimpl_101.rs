// Generated macro for impl_101 (impl)
macro_rules! Depcrate_relative_pathimpl_101 {
() => {
// Module: crate::relative_path
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for & 'a RelativePath { type Error = Error ; # [inline] fn try_from (value : & 'a [u8]) -> Result < Self , Self :: Error > { let path = try_from_byte_slice (value) ? ; relative_path_from_value_and_path (value . as_bstr () , path) } }
};
}
