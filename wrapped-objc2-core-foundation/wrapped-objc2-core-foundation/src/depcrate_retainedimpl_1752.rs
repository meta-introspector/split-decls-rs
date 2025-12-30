// Generated macro for impl_1752 (impl)
macro_rules! Depcrate_retainedimpl_1752 {
() => {
// Module: crate::retained
// Provides: {"impl_1752"}
// Dependencies: {}
impl < T : ? Sized + ConcreteType + 'static > From < CFRetained < T > > for CFRetained < CFType > { # [doc = " Convert to [`CFType`]."] # [inline] fn from (obj : CFRetained < T >) -> Self { unsafe { CFRetained :: cast_unchecked (obj) } } }
};
}
