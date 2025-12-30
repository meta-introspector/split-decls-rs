// Generated macro for impl_217 (impl)
macro_rules! Depcrate_refs_globalimpl_217 {
() => {
// Module: crate::refs::global
// Provides: {"impl_217"}
// Dependencies: {}
impl < T > Deref for Global < T > where T : Into < JObject < 'static > > + AsRef < JObject < 'static > > + Default + Reference + Send + Sync + 'static , { type Target = T ; fn deref (& self) -> & Self :: Target { & self . obj } }
};
}
