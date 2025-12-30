// Generated macro for impl_214 (impl)
macro_rules! Depcrate_numberimpl_214 {
() => {
// Module: crate::number
// Provides: {"impl_214"}
// Dependencies: {}
impl From < i64 > for CFNumber { # [inline] fn from (value : i64) -> Self { unsafe { let number_ref = CFNumberCreate (kCFAllocatorDefault , kCFNumberSInt64Type , & value as * const i64 as * const c_void ,) ; TCFType :: wrap_under_create_rule (number_ref) } } }
};
}
