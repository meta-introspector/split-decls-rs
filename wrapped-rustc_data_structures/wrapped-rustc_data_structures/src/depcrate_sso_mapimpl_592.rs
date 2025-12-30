// Generated macro for impl_592 (impl)
macro_rules! Depcrate_sso_mapimpl_592 {
() => {
// Module: crate::sso::map
// Provides: {"impl_592"}
// Dependencies: {}
impl < K : Eq + Hash , V > FromIterator < (K , V) > for SsoHashMap < K , V > { fn from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> SsoHashMap < K , V > { let mut map : SsoHashMap < K , V > = Default :: default () ; map . extend (iter) ; map } }
};
}
