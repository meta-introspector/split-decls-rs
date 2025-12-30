// Generated macro for impl_108 (impl)
macro_rules! Depcrate_object_convertimpl_108 {
() => {
// Module: crate::object::convert
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'a > TryFrom < ObjectRef < 'a > > for CommitRef < 'a > { type Error = ObjectRef < 'a > ; fn try_from (value : ObjectRef < 'a >) -> Result < Self , Self :: Error > { Ok (match value { ObjectRef :: Commit (v) => v , _ => return Err (value) , }) } }
};
}
