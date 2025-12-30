// Generated macro for impl_137 (impl)
macro_rules! Depcrate_com_objectimpl_137 {
() => {
// Module: crate::com_object
// Provides: {"impl_137"}
// Dependencies: {}
impl < T : ComObjectInner > Drop for ComObject < T > { fn drop (& mut self) { unsafe { T :: Outer :: Release (self . ptr . as_ptr ()) ; } } }
};
}
