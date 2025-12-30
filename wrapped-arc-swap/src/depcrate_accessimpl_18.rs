// Generated macro for impl_18 (impl)
macro_rules! Depcrate_accessimpl_18 {
() => {
// Module: crate::access
// Provides: {"impl_18"}
// Dependencies: {}
impl < T , S : Strategy < Arc < T > > > Deref for DirectDeref < Arc < T > , S > { type Target = T ; fn deref (& self) -> & T { self . 0 . deref () . deref () } }
};
}
