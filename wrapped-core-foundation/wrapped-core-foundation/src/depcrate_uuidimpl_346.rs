// Generated macro for impl_346 (impl)
macro_rules! Depcrate_uuidimpl_346 {
() => {
// Module: crate::uuid
// Provides: {"impl_346"}
// Dependencies: {}
impl CFUUID { # [inline] pub fn new () -> CFUUID { unsafe { let uuid_ref = CFUUIDCreate (kCFAllocatorDefault) ; TCFType :: wrap_under_create_rule (uuid_ref) } } }
};
}
