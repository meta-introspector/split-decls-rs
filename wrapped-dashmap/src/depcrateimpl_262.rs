// Generated macro for impl_262 (impl)
macro_rules! Depcrateimpl_262 {
() => {
// Module: crate
// Provides: {"impl_262"}
// Dependencies: {}
impl < K : Eq + Hash , V , S : BuildHasher + Clone + Default > FromIterator < (K , V) > for DashMap < K , V , S > { fn from_iter < I : IntoIterator < Item = (K , V) > > (intoiter : I) -> Self { let mut map = DashMap :: default () ; map . extend (intoiter) ; map } }
};
}
