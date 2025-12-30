// Generated macro for impl_494 (impl)
macro_rules! Depcrate_setimpl_494 {
() => {
// Module: crate::set
// Provides: {"impl_494"}
// Dependencies: {}
impl < T , S , A > BitOr < & HashSet < T , S , A > > for & HashSet < T , S , A > where T : Eq + Hash + Clone , S : BuildHasher + Default , A : Allocator + Default , { type Output = HashSet < T , S , A > ; # [doc = " Returns the union of `self` and `rhs` as a new `HashSet<T, S>`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashSet;"] # [doc = ""] # [doc = " let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();"] # [doc = " let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();"] # [doc = ""] # [doc = " let set = &a | &b;"] # [doc = ""] # [doc = " let mut i = 0;"] # [doc = " let expected = [1, 2, 3, 4, 5];"] # [doc = " for x in &set {"] # [doc = "     assert!(expected.contains(x));"] # [doc = "     i += 1;"] # [doc = " }"] # [doc = " assert_eq!(i, expected.len());"] # [doc = " ```"] fn bitor (self , rhs : & HashSet < T , S , A >) -> HashSet < T , S , A > { self . union (rhs) . cloned () . collect () } }
};
}
