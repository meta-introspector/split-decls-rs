// Generated macro for impl_114 (impl)
macro_rules! Depcrate_mapref_oneimpl_114 {
() => {
// Module: crate::mapref::one
// Provides: {"impl_114"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , T : ? Sized > DerefMut for MappedRefMut < 'a , K , T > { fn deref_mut (& mut self) -> & mut T { self . value_mut () } }
};
}
