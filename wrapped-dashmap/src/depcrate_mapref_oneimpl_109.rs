// Generated macro for impl_109 (impl)
macro_rules! Depcrate_mapref_oneimpl_109 {
() => {
// Module: crate::mapref::one
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , T : ? Sized + AsRef < TDeref > , TDeref : ? Sized > AsRef < TDeref > for MappedRef < 'a , K , T > { fn as_ref (& self) -> & TDeref { self . value () . as_ref () } }
};
}
