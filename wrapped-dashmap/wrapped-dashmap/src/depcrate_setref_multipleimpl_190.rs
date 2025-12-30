// Generated macro for impl_190 (impl)
macro_rules! Depcrate_setref_multipleimpl_190 {
() => {
// Module: crate::setref::multiple
// Provides: {"impl_190"}
// Dependencies: {}
impl < 'a , K : Eq + Hash > RefMulti < 'a , K > { pub (crate) fn new (inner : mapref :: multiple :: RefMulti < 'a , K , () >) -> Self { Self { inner } } pub fn key (& self) -> & K { self . inner . key () } }
};
}
