// Generated macro for impl_253 (impl)
macro_rules! Depcrate_refs_autoimpl_253 {
() => {
// Module: crate::refs::auto
// Provides: {"impl_253"}
// Dependencies: {}
impl < 'local , T > Deref for Auto < 'local , T > where T : Into < JObject < 'local > > , { type Target = T ; fn deref (& self) -> & Self :: Target { & self . obj } }
};
}
