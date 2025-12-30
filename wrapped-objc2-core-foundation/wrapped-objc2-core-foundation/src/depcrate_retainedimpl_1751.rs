// Generated macro for impl_1751 (impl)
macro_rules! Depcrate_retainedimpl_1751 {
() => {
// Module: crate::retained
// Provides: {"impl_1751"}
// Dependencies: {}
impl < T : ? Sized + AsRef < U > , U : Type > From < & T > for CFRetained < U > { # [doc = " Cast the type to a superclass or `CFType`, and retain it."] # [inline] fn from (obj : & T) -> Self { obj . as_ref () . retain () } }
};
}
