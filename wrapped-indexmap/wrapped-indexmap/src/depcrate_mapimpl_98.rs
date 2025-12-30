// Generated macro for impl_98 (impl)
macro_rules! Depcrate_mapimpl_98 {
() => {
// Module: crate::map
// Provides: {"impl_98"}
// Dependencies: {}
impl < K , V , S > FromIterator < (K , V) > for IndexMap < K , V , S > where K : Hash + Eq , S : BuildHasher + Default , { # [doc = " Create an `IndexMap` from the sequence of key-value pairs in the"] # [doc = " iterable."] # [doc = ""] # [doc = " `from_iter` uses the same logic as `extend`. See"] # [doc = " [`extend`][IndexMap::extend] for more details."] fn from_iter < I : IntoIterator < Item = (K , V) > > (iterable : I) -> Self { let iter = iterable . into_iter () ; let (low , _) = iter . size_hint () ; let mut map = Self :: with_capacity_and_hasher (low , < _ > :: default ()) ; map . extend (iter) ; map } }
};
}
