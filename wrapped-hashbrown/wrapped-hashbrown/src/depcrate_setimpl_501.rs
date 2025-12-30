// Generated macro for impl_501 (impl)
macro_rules! Depcrate_setimpl_501 {
() => {
// Module: crate::set
// Provides: {"impl_501"}
// Dependencies: {}
impl < T , S , A > SubAssign < & HashSet < T , S , A > > for HashSet < T , S , A > where T : Eq + Hash + Clone , S : BuildHasher , A : Allocator , { # [doc = " Modifies this set to contain the difference of `self` and `rhs`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashSet;"] # [doc = ""] # [doc = " let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();"] # [doc = " let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();"] # [doc = ""] # [doc = " a -= &b;"] # [doc = ""] # [doc = " let mut i = 0;"] # [doc = " let expected = [1, 2];"] # [doc = " for x in &a {"] # [doc = "     assert!(expected.contains(x));"] # [doc = "     i += 1;"] # [doc = " }"] # [doc = " assert_eq!(i, expected.len());"] # [doc = " ```"] fn sub_assign (& mut self , rhs : & HashSet < T , S , A >) { if rhs . len () < self . len () { for item in rhs { self . remove (item) ; } } else { self . retain (| item | ! rhs . contains (item)) ; } } }
};
}
