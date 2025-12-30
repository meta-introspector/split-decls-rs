// Generated macro for impl_88 (impl)
macro_rules! Depcrate_mapref_multipleimpl_88 {
() => {
// Module: crate::mapref::multiple
// Provides: {"impl_88"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , V > Deref for RefMutMulti < 'a , K , V > { type Target = V ; fn deref (& self) -> & V { self . value () } }
};
}
