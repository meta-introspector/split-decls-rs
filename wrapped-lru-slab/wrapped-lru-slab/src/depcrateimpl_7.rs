// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl < T > FromIterator < T > for LruSlab < T > { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { let iter = iter . into_iter () ; let mut slab = LruSlab :: with_capacity (u32 :: try_from (iter . size_hint () . 0) . unwrap ()) ; for x in iter { slab . insert (x) ; } slab } }
};
}
