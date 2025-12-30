// Generated macro for impl_213 (impl)
macro_rules! Depcrate_numberimpl_213 {
() => {
// Module: crate::number
// Provides: {"impl_213"}
// Dependencies: {}
impl From < i32 > for CFNumber { # [inline] fn from (value : i32) -> Self { unsafe { let number_ref = CFNumberCreate (kCFAllocatorDefault , kCFNumberSInt32Type , & value as * const i32 as * const c_void ,) ; TCFType :: wrap_under_create_rule (number_ref) } } }
};
}
