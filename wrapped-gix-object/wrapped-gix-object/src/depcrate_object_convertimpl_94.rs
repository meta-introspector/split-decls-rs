// Generated macro for impl_94 (impl)
macro_rules! Depcrate_object_convertimpl_94 {
() => {
// Module: crate::object::convert
// Provides: {"impl_94"}
// Dependencies: {}
impl TryFrom < ObjectRef < '_ > > for Object { type Error = crate :: decode :: Error ; fn try_from (v : ObjectRef < '_ >) -> Result < Self , Self :: Error > { Ok (match v { ObjectRef :: Tree (v) => Object :: Tree (v . into ()) , ObjectRef :: Blob (v) => Object :: Blob (v . into ()) , ObjectRef :: Commit (v) => Object :: Commit (v . try_into () ?) , ObjectRef :: Tag (v) => Object :: Tag (v . try_into () ?) , }) } }
};
}
