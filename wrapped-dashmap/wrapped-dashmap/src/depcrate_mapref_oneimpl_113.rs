// Generated macro for impl_113 (impl)
macro_rules! Depcrate_mapref_oneimpl_113 {
() => {
// Module: crate::mapref::one
// Provides: {"impl_113"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , T : ? Sized > Deref for MappedRefMut < 'a , K , T > { type Target = T ; fn deref (& self) -> & T { self . value () } }
};
}
