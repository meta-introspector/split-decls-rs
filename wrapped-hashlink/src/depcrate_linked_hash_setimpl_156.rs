// Generated macro for impl_156 (impl)
macro_rules! Depcrate_linked_hash_setimpl_156 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_156"}
// Dependencies: {}
impl < T , S > BitXor < & LinkedHashSet < T , S > > for & LinkedHashSet < T , S > where T : Eq + Hash + Clone , S : BuildHasher + Default , { type Output = LinkedHashSet < T , S > ; # [inline] fn bitxor (self , rhs : & LinkedHashSet < T , S >) -> LinkedHashSet < T , S > { self . symmetric_difference (rhs) . cloned () . collect () } }
};
}
