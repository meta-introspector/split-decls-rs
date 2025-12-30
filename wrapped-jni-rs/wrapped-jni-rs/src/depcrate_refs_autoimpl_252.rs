// Generated macro for impl_252 (impl)
macro_rules! Depcrate_refs_autoimpl_252 {
() => {
// Module: crate::refs::auto
// Provides: {"impl_252"}
// Dependencies: {}
impl < 'local , T , U > AsRef < U > for Auto < 'local , T > where T : AsRef < U > + Into < JObject < 'local > > , { fn as_ref (& self) -> & U { self . obj . as_ref () } }
};
}
