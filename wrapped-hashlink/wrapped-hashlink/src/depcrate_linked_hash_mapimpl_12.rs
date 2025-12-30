// Generated macro for impl_12 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_12 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_12"}
// Dependencies: {}
impl < K , V , S > LinkedHashMap < K , V , S > where S : BuildHasher , { # [inline] pub fn raw_entry (& self) -> RawEntryBuilder < '_ , K , V , S > { RawEntryBuilder { map : self } } # [inline] pub fn raw_entry_mut (& mut self) -> RawEntryBuilderMut < '_ , K , V , S > { RawEntryBuilderMut { map : self } } }
};
}
