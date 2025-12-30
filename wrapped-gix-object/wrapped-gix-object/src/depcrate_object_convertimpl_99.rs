// Generated macro for impl_99 (impl)
macro_rules! Depcrate_object_convertimpl_99 {
() => {
// Module: crate::object::convert
// Provides: {"impl_99"}
// Dependencies: {}
impl TryFrom < Object > for Tag { type Error = Object ; fn try_from (value : Object) -> Result < Self , Self :: Error > { Ok (match value { Object :: Tag (v) => v , _ => return Err (value) , }) } }
};
}
