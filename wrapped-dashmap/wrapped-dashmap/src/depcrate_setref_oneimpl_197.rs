// Generated macro for impl_197 (impl)
macro_rules! Depcrate_setref_oneimpl_197 {
() => {
// Module: crate::setref::one
// Provides: {"impl_197"}
// Dependencies: {}
impl < 'a , K : Eq + Hash > Ref < 'a , K > { pub (crate) fn new (inner : mapref :: one :: Ref < 'a , K , () >) -> Self { Self { inner } } pub fn key (& self) -> & K { self . inner . key () } }
};
}
