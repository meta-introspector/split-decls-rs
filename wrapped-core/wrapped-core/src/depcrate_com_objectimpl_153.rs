// Generated macro for impl_153 (impl)
macro_rules! Depcrate_com_objectimpl_153 {
() => {
// Module: crate::com_object
// Provides: {"impl_153"}
// Dependencies: {}
impl < T > StaticComObject < T > where T : ComObjectInner , { # [doc = " Wraps `outer` in a `StaticComObject`."] pub const fn from_outer (outer : T :: Outer) -> Self { Self { outer } } }
};
}
