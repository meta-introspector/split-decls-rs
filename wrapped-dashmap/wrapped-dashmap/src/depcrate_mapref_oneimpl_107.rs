// Generated macro for impl_107 (impl)
macro_rules! Depcrate_mapref_oneimpl_107 {
() => {
// Module: crate::mapref::one
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , T : ? Sized > Deref for MappedRef < 'a , K , T > { type Target = T ; fn deref (& self) -> & T { self . value () } }
};
}
