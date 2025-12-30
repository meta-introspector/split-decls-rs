// Generated macro for impl_89 (impl)
macro_rules! Depcrate_mapref_multipleimpl_89 {
() => {
// Module: crate::mapref::multiple
// Provides: {"impl_89"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , V > DerefMut for RefMutMulti < 'a , K , V > { fn deref_mut (& mut self) -> & mut V { self . value_mut () } }
};
}
