// Generated macro for impl_99 (impl)
macro_rules! Depcrate_relative_pathimpl_99 {
() => {
// Module: crate::relative_path
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a str > for & 'a RelativePath { type Error = Error ; fn try_from (value : & 'a str) -> Result < Self , Self :: Error > { relative_path_from_value_and_path (value . into () , Path :: new (value)) } }
};
}
