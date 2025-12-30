// Generated macro for impl_236 (impl)
macro_rules! Depcrate_refs_weakimpl_236 {
() => {
// Module: crate::refs::weak
// Provides: {"impl_236"}
// Dependencies: {}
impl < T > Deref for Weak < T > where T : Into < JObject < 'static > > + AsRef < JObject < 'static > > + Default + Reference + Send + Sync , { type Target = T ; fn deref (& self) -> & Self :: Target { & self . obj } }
};
}
