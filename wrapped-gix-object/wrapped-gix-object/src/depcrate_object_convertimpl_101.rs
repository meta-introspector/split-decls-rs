// Generated macro for impl_101 (impl)
macro_rules! Depcrate_object_convertimpl_101 {
() => {
// Module: crate::object::convert
// Provides: {"impl_101"}
// Dependencies: {}
impl TryFrom < Object > for Tree { type Error = Object ; fn try_from (value : Object) -> Result < Self , Self :: Error > { Ok (match value { Object :: Tree (v) => v , _ => return Err (value) , }) } }
};
}
