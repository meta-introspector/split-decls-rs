// Generated macro for impl_103 (impl)
macro_rules! Depcrate_mapref_oneimpl_103 {
() => {
// Module: crate::mapref::one
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , V > DerefMut for RefMut < 'a , K , V > { fn deref_mut (& mut self) -> & mut V { self . value_mut () } }
};
}
