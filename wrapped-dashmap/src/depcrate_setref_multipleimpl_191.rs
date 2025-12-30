// Generated macro for impl_191 (impl)
macro_rules! Depcrate_setref_multipleimpl_191 {
() => {
// Module: crate::setref::multiple
// Provides: {"impl_191"}
// Dependencies: {}
impl < 'a , K : Eq + Hash > Deref for RefMulti < 'a , K > { type Target = K ; fn deref (& self) -> & K { self . key () } }
};
}
