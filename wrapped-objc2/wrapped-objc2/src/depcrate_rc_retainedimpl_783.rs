// Generated macro for impl_783 (impl)
macro_rules! Depcrate_rc_retainedimpl_783 {
() => {
// Module: crate::rc::retained
// Provides: {"impl_783"}
// Dependencies: {}
impl < T : ? Sized + AsRef < U > , U : Message > From < & T > for Retained < U > { # [doc = " Cast the object to its superclass, and retain it."] # [inline] fn from (obj : & T) -> Self { obj . as_ref () . retain () } }
};
}
