// Generated macro for impl_200 (impl)
macro_rules! Depcrate_retainedimpl_200 {
() => {
// Module: crate::retained
// Provides: {"impl_200"}
// Dependencies: {}
impl < T : ? Sized + AsRef < U > , U : DispatchObject > From < & T > for DispatchRetained < U > { # [doc = " Cast the object to a superclass, and retain it."] # [inline] fn from (obj : & T) -> Self { obj . as_ref () . retain () } }
};
}
