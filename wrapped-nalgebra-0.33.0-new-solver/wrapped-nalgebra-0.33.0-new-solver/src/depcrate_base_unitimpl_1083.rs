// Generated macro for impl_1083 (impl)
macro_rules! Depcrate_base_unitimpl_1083 {
() => {
// Module: crate::base::unit
// Provides: {"impl_1083"}
// Dependencies: {}
impl < T > Deref for Unit < T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * (self as * const Self as * const T) } } }
};
}
