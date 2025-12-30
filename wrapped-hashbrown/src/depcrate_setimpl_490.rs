// Generated macro for impl_490 (impl)
macro_rules! Depcrate_setimpl_490 {
() => {
// Module: crate::set
// Provides: {"impl_490"}
// Dependencies: {}
# [cfg (feature = "default-hasher")] impl < T , A , const N : usize > From < [T ; N] > for HashSet < T , DefaultHashBuilder , A > where T : Eq + Hash , A : Default + Allocator , { # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashSet;"] # [doc = ""] # [doc = " let set1 = HashSet::from([1, 2, 3, 4]);"] # [doc = " let set2: HashSet<_> = [1, 2, 3, 4].into();"] # [doc = " assert_eq!(set1, set2);"] # [doc = " ```"] fn from (arr : [T ; N]) -> Self { arr . into_iter () . collect () } }
};
}
