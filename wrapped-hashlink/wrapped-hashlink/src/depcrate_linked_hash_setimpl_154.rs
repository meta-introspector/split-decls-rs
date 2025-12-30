// Generated macro for impl_154 (impl)
macro_rules! Depcrate_linked_hash_setimpl_154 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_154"}
// Dependencies: {}
impl < T , S > BitOr < & LinkedHashSet < T , S > > for & LinkedHashSet < T , S > where T : Eq + Hash + Clone , S : BuildHasher + Default , { type Output = LinkedHashSet < T , S > ; # [inline] fn bitor (self , rhs : & LinkedHashSet < T , S >) -> LinkedHashSet < T , S > { self . union (rhs) . cloned () . collect () } }
};
}
