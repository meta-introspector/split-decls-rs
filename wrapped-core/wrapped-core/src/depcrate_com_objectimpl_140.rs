// Generated macro for impl_140 (impl)
macro_rules! Depcrate_com_objectimpl_140 {
() => {
// Module: crate::com_object
// Provides: {"impl_140"}
// Dependencies: {}
impl < T : ComObjectInner > Deref for ComObject < T > { type Target = T :: Outer ; # [inline (always)] fn deref (& self) -> & Self :: Target { self . get_box () } }
};
}
