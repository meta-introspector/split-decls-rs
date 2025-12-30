// Generated macro for impl_18 (impl)
macro_rules! Depcrate_object_idimpl_18 {
() => {
// Module: crate::object_id
// Provides: {"impl_18"}
// Dependencies: {}
# [allow (clippy :: derived_hash_with_manual_eq)] impl Hash for ObjectId { fn hash < H : Hasher > (& self , state : & mut H) { state . write (self . as_slice ()) ; } }
};
}
