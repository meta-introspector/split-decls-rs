// Generated macro for impl_157 (impl)
macro_rules! Depcrate_linked_hash_setimpl_157 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_157"}
// Dependencies: {}
impl < T , S > Sub < & LinkedHashSet < T , S > > for & LinkedHashSet < T , S > where T : Eq + Hash + Clone , S : BuildHasher + Default , { type Output = LinkedHashSet < T , S > ; # [inline] fn sub (self , rhs : & LinkedHashSet < T , S >) -> LinkedHashSet < T , S > { self . difference (rhs) . cloned () . collect () } }
};
}
