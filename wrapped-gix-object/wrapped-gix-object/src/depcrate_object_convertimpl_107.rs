// Generated macro for impl_107 (impl)
macro_rules! Depcrate_object_convertimpl_107 {
() => {
// Module: crate::object::convert
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'a > TryFrom < ObjectRef < 'a > > for TagRef < 'a > { type Error = ObjectRef < 'a > ; fn try_from (value : ObjectRef < 'a >) -> Result < Self , Self :: Error > { Ok (match value { ObjectRef :: Tag (v) => v , _ => return Err (value) , }) } }
};
}
