// Generated macro for impl_247 (impl)
macro_rules! Depcrateimpl_247 {
() => {
// Module: crate
// Provides: {"impl_247"}
// Dependencies: {}
impl < K , V , S > Default for DashMap < K , V , S > where K : Eq + Hash , S : Default + BuildHasher + Clone , { fn default () -> Self { Self :: with_hasher (Default :: default ()) } }
};
}
