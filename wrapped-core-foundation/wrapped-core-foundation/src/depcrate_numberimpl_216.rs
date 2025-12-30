// Generated macro for impl_216 (impl)
macro_rules! Depcrate_numberimpl_216 {
() => {
// Module: crate::number
// Provides: {"impl_216"}
// Dependencies: {}
impl From < f64 > for CFNumber { # [inline] fn from (value : f64) -> Self { unsafe { let number_ref = CFNumberCreate (kCFAllocatorDefault , kCFNumberFloat64Type , & value as * const f64 as * const c_void ,) ; TCFType :: wrap_under_create_rule (number_ref) } } }
};
}
