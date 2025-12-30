// Generated macro for impl_102 (impl)
macro_rules! Depcrate_object_convertimpl_102 {
() => {
// Module: crate::object::convert
// Provides: {"impl_102"}
// Dependencies: {}
impl TryFrom < Object > for Blob { type Error = Object ; fn try_from (value : Object) -> Result < Self , Self :: Error > { Ok (match value { Object :: Blob (v) => v , _ => return Err (value) , }) } }
};
}
