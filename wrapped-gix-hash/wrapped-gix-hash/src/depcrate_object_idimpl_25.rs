// Generated macro for impl_25 (impl)
macro_rules! Depcrate_object_idimpl_25 {
() => {
// Module: crate::object_id
// Provides: {"impl_25"}
// Dependencies: {}
impl From < & oid > for ObjectId { fn from (v : & oid) -> Self { match v . kind () { Kind :: Sha1 => ObjectId :: from_20_bytes (v . as_bytes ()) , } } }
};
}
