// Generated macro for impl_110 (impl)
macro_rules! Depcrate_object_convertimpl_110 {
() => {
// Module: crate::object::convert
// Provides: {"impl_110"}
// Dependencies: {}
impl < 'a > TryFrom < ObjectRef < 'a > > for BlobRef < 'a > { type Error = ObjectRef < 'a > ; fn try_from (value : ObjectRef < 'a >) -> Result < Self , Self :: Error > { Ok (match value { ObjectRef :: Blob (v) => v , _ => return Err (value) , }) } }
};
}
