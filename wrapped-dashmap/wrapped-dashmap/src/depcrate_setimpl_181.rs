// Generated macro for impl_181 (impl)
macro_rules! Depcrate_setimpl_181 {
() => {
// Module: crate::set
// Provides: {"impl_181"}
// Dependencies: {}
impl < K : Eq + Hash , S : BuildHasher + Clone + Default > FromIterator < K > for DashSet < K , S > { fn from_iter < I : IntoIterator < Item = K > > (iter : I) -> Self { let mut set = DashSet :: default () ; set . extend (iter) ; set } }
};
}
