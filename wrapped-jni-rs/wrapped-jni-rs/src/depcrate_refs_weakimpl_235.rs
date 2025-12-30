// Generated macro for impl_235 (impl)
macro_rules! Depcrate_refs_weakimpl_235 {
() => {
// Module: crate::refs::weak
// Provides: {"impl_235"}
// Dependencies: {}
impl < T , U > AsRef < U > for Weak < T > where T : AsRef < U > + Into < JObject < 'static > > + AsRef < JObject < 'static > > + Default + Reference + Send + Sync , { fn as_ref (& self) -> & U { self . obj . as_ref () } }
};
}
