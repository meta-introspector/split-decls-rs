// Generated macro for impl_234 (impl)
macro_rules! Depcrate_refs_weakimpl_234 {
() => {
// Module: crate::refs::weak
// Provides: {"impl_234"}
// Dependencies: {}
impl < T > Default for Weak < T > where T : Into < JObject < 'static > > + AsRef < JObject < 'static > > + Default + Reference + Send + Sync , { fn default () -> Self { Self :: null () } }
};
}
