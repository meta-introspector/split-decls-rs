// Generated macro for impl_102 (impl)
macro_rules! Depcrate_mapref_oneimpl_102 {
() => {
// Module: crate::mapref::one
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , V > Deref for RefMut < 'a , K , V > { type Target = V ; fn deref (& self) -> & V { self . value () } }
};
}
