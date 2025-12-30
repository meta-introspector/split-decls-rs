// Generated macro for impl_138 (impl)
macro_rules! Depcrate_com_objectimpl_138 {
() => {
// Module: crate::com_object
// Provides: {"impl_138"}
// Dependencies: {}
impl < T : ComObjectInner > Clone for ComObject < T > { # [inline (always)] fn clone (& self) -> Self { unsafe { self . ptr . as_ref () . AddRef () ; Self { ptr : self . ptr } } } }
};
}
