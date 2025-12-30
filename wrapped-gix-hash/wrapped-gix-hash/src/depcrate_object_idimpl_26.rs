// Generated macro for impl_26 (impl)
macro_rules! Depcrate_object_idimpl_26 {
() => {
// Module: crate::object_id
// Provides: {"impl_26"}
// Dependencies: {}
impl TryFrom < & [u8] > for ObjectId { type Error = crate :: Error ; fn try_from (bytes : & [u8]) -> Result < Self , Self :: Error > { Ok (oid :: try_from_bytes (bytes) ? . into ()) } }
};
}
