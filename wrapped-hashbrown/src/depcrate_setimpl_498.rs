// Generated macro for impl_498 (impl)
macro_rules! Depcrate_setimpl_498 {
() => {
// Module: crate::set
// Provides: {"impl_498"}
// Dependencies: {}
impl < T , S , A > BitOrAssign < & HashSet < T , S , A > > for HashSet < T , S , A > where T : Eq + Hash + Clone , S : BuildHasher , A : Allocator , { # [doc = " Modifies this set to contain the union of `self` and `rhs`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashSet;"] # [doc = ""] # [doc = " let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();"] # [doc = " let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();"] # [doc = ""] # [doc = " a |= &b;"] # [doc = ""] # [doc = " let mut i = 0;"] # [doc = " let expected = [1, 2, 3, 4, 5];"] # [doc = " for x in &a {"] # [doc = "     assert!(expected.contains(x));"] # [doc = "     i += 1;"] # [doc = " }"] # [doc = " assert_eq!(i, expected.len());"] # [doc = " ```"] fn bitor_assign (& mut self , rhs : & HashSet < T , S , A >) { for item in rhs { if ! self . contains (item) { self . insert (item . clone ()) ; } } } }
};
}
