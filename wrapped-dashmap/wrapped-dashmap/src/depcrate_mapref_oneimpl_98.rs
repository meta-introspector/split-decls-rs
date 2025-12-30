// Generated macro for impl_98 (impl)
macro_rules! Depcrate_mapref_oneimpl_98 {
() => {
// Module: crate::mapref::one
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , V > Deref for Ref < 'a , K , V > { type Target = V ; fn deref (& self) -> & V { self . value () } }
};
}
