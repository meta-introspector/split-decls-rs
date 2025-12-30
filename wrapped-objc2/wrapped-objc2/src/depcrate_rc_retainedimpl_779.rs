// Generated macro for impl_779 (impl)
macro_rules! Depcrate_rc_retainedimpl_779 {
() => {
// Module: crate::rc::retained
// Provides: {"impl_779"}
// Dependencies: {}
impl < T : Message > Clone for Retained < T > { # [doc = " Retain the object, increasing its reference count."] # [doc = ""] # [doc = " This is equivalent to [`Message::retain`]."] # [doc (alias = "objc_retain")] # [doc (alias = "retain")] # [inline] fn clone (& self) -> Self { self . retain () } }
};
}
