// Generated macro for impl_102 (impl)
macro_rules! Depcrate_relative_pathimpl_102 {
() => {
// Module: crate::relative_path
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'a , const N : usize > TryFrom < & 'a [u8 ; N] > for & 'a RelativePath { type Error = Error ; # [inline] fn try_from (value : & 'a [u8 ; N]) -> Result < Self , Self :: Error > { let path = try_from_byte_slice (value . as_bstr ()) ? ; relative_path_from_value_and_path (value . as_bstr () , path) } }
};
}
