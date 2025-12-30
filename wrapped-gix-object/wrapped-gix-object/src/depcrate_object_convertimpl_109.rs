// Generated macro for impl_109 (impl)
macro_rules! Depcrate_object_convertimpl_109 {
() => {
// Module: crate::object::convert
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'a > TryFrom < ObjectRef < 'a > > for TreeRef < 'a > { type Error = ObjectRef < 'a > ; fn try_from (value : ObjectRef < 'a >) -> Result < Self , Self :: Error > { Ok (match value { ObjectRef :: Tree (v) => v , _ => return Err (value) , }) } }
};
}
