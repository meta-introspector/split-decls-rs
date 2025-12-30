// Generated macro for impl_154 (impl)
macro_rules! Depcrate_com_objectimpl_154 {
() => {
// Module: crate::com_object
// Provides: {"impl_154"}
// Dependencies: {}
impl < T > StaticComObject < T > where T : ComObjectInner , { # [doc = " Gets access to the contained value."] pub const fn get (& 'static self) -> & 'static T :: Outer { & self . outer } }
};
}
