// Generated macro for impl_215 (impl)
macro_rules! Depcrate_numberimpl_215 {
() => {
// Module: crate::number
// Provides: {"impl_215"}
// Dependencies: {}
impl From < f32 > for CFNumber { # [inline] fn from (value : f32) -> Self { unsafe { let number_ref = CFNumberCreate (kCFAllocatorDefault , kCFNumberFloat32Type , & value as * const f32 as * const c_void ,) ; TCFType :: wrap_under_create_rule (number_ref) } } }
};
}
