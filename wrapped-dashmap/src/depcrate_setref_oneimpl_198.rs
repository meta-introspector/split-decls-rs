// Generated macro for impl_198 (impl)
macro_rules! Depcrate_setref_oneimpl_198 {
() => {
// Module: crate::setref::one
// Provides: {"impl_198"}
// Dependencies: {}
impl < 'a , K : Eq + Hash > Deref for Ref < 'a , K > { type Target = K ; fn deref (& self) -> & K { self . key () } }
};
}
