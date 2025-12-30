// Generated macro for impl_20 (impl)
macro_rules! Depcrate_accessimpl_20 {
() => {
// Module: crate::access
// Provides: {"impl_20"}
// Dependencies: {}
impl < T , S : Strategy < Rc < T > > > Deref for DirectDeref < Rc < T > , S > { type Target = T ; fn deref (& self) -> & T { self . 0 . deref () . deref () } }
};
}
