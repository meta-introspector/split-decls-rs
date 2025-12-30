// Generated macro for impl_411 (impl)
macro_rules! Depcrate_hash_setimpl_411 {
() => {
// Module: crate::hash::set
// Provides: {"impl_411"}
// Dependencies: {}
impl < A , S , R > Extend < R > for HashSet < A , S > where A : Hash + Eq + Clone + From < R > , S : BuildHasher , { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = R > , { for value in iter { self . insert (From :: from (value)) ; } } }
};
}
