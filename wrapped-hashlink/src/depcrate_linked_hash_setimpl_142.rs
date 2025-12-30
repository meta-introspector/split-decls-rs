// Generated macro for impl_142 (impl)
macro_rules! Depcrate_linked_hash_setimpl_142 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_142"}
// Dependencies: {}
impl < T : Hash + Eq > LinkedHashSet < T , DefaultHashBuilder > { # [inline] pub fn new () -> LinkedHashSet < T , DefaultHashBuilder > { LinkedHashSet { map : LinkedHashMap :: new () , } } # [inline] pub fn with_capacity (capacity : usize) -> LinkedHashSet < T , DefaultHashBuilder > { LinkedHashSet { map : LinkedHashMap :: with_capacity (capacity) , } } }
};
}
