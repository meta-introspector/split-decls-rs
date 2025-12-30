// Generated macro for impl_9 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_9 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_9"}
// Dependencies: {}
impl < K , V > LinkedHashMap < K , V > { # [inline] pub fn new () -> Self { Self { hash_builder : DefaultHashBuilder :: default () , table : HashTable :: new () , values : None , free : None , } } # [inline] pub fn with_capacity (capacity : usize) -> Self { Self { hash_builder : DefaultHashBuilder :: default () , table : HashTable :: with_capacity (capacity) , values : None , free : None , } } }
};
}
