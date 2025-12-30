// Generated macro for impl_174 (impl)
macro_rules! Depcrate_setimpl_174 {
() => {
// Module: crate::set
// Provides: {"impl_174"}
// Dependencies: {}
impl < K , S > Default for DashSet < K , S > where K : Eq + Hash , S : Default + BuildHasher + Clone , { fn default () -> Self { Self :: with_hasher (Default :: default ()) } }
};
}
