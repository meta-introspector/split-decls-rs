// Generated macro for impl_489 (impl)
macro_rules! Depcrate_setimpl_489 {
() => {
// Module: crate::set
// Provides: {"impl_489"}
// Dependencies: {}
impl < T , S , A > FromIterator < T > for HashSet < T , S , A > where T : Eq + Hash , S : BuildHasher + Default , A : Default + Allocator , { # [cfg_attr (feature = "inline-more" , inline)] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { let mut set = Self :: with_hasher_in (Default :: default () , Default :: default ()) ; set . extend (iter) ; set } }
};
}
