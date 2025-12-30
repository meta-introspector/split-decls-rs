// Generated macro for impl_142 (impl)
macro_rules! Depcrate_dateimpl_142 {
() => {
// Module: crate::date
// Provides: {"impl_142"}
// Dependencies: {}
impl CFDate { # [inline] pub fn new (time : CFAbsoluteTime) -> CFDate { unsafe { let date_ref = CFDateCreate (kCFAllocatorDefault , time) ; TCFType :: wrap_under_create_rule (date_ref) } } # [inline] pub fn now () -> CFDate { CFDate :: new (unsafe { CFAbsoluteTimeGetCurrent () }) } # [inline] pub fn abs_time (& self) -> CFAbsoluteTime { unsafe { CFDateGetAbsoluteTime (self . 0) } } }
};
}
