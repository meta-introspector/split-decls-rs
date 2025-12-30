// Generated macro for impl_423 (impl)
macro_rules! Depcrate_hash_setimpl_423 {
() => {
// Module: crate::hash::set
// Provides: {"impl_423"}
// Dependencies: {}
impl < A , RA , S > FromIterator < RA > for HashSet < A , S > where A : Hash + Eq + Clone + From < RA > , S : BuildHasher + Default , { fn from_iter < T > (i : T) -> Self where T : IntoIterator < Item = RA > , { let mut set = Self :: default () ; for value in i { set . insert (From :: from (value)) ; } set } }
};
}
