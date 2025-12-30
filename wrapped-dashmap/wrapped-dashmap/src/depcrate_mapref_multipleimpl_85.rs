// Generated macro for impl_85 (impl)
macro_rules! Depcrate_mapref_multipleimpl_85 {
() => {
// Module: crate::mapref::multiple
// Provides: {"impl_85"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , V > Deref for RefMulti < 'a , K , V > { type Target = V ; fn deref (& self) -> & V { self . value () } }
};
}
