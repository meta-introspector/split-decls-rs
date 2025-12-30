// Generated macro for impl_100 (impl)
macro_rules! Depcrate_object_convertimpl_100 {
() => {
// Module: crate::object::convert
// Provides: {"impl_100"}
// Dependencies: {}
impl TryFrom < Object > for Commit { type Error = Object ; fn try_from (value : Object) -> Result < Self , Self :: Error > { Ok (match value { Object :: Commit (v) => v , _ => return Err (value) , }) } }
};
}
