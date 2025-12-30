// Generated macro for impl_155 (impl)
macro_rules! Depcrate_linked_hash_setimpl_155 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_155"}
// Dependencies: {}
impl < T , S > BitAnd < & LinkedHashSet < T , S > > for & LinkedHashSet < T , S > where T : Eq + Hash + Clone , S : BuildHasher + Default , { type Output = LinkedHashSet < T , S > ; # [inline] fn bitand (self , rhs : & LinkedHashSet < T , S >) -> LinkedHashSet < T , S > { self . intersection (rhs) . cloned () . collect () } }
};
}
