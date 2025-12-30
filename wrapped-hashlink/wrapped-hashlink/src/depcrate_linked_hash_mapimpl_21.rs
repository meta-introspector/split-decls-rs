// Generated macro for impl_21 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_21 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_21"}
// Dependencies: {}
impl < K , V , S > Drop for LinkedHashMap < K , V , S > { # [inline] fn drop (& mut self) { unsafe { if let Some (values) = self . values { drop_value_nodes (values) ; let _ = Box :: from_raw (values . as_ptr ()) ; } drop_free_nodes (self . free) ; } } }
};
}
