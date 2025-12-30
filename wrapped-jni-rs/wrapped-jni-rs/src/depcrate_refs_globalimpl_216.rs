// Generated macro for impl_216 (impl)
macro_rules! Depcrate_refs_globalimpl_216 {
() => {
// Module: crate::refs::global
// Provides: {"impl_216"}
// Dependencies: {}
impl < T , U > AsRef < U > for Global < T > where T : AsRef < U > + Into < JObject < 'static > > + AsRef < JObject < 'static > > + Default + Reference + Send + Sync , { fn as_ref (& self) -> & U { self . obj . as_ref () } }
};
}
