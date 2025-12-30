// Generated macro for impl_179 (impl)
macro_rules! Depcrate_errorimpl_179 {
() => {
// Module: crate::error
// Provides: {"impl_179"}
// Dependencies: {}
impl CFError { # [doc = " Returns a string identifying the domain with which this error is"] # [doc = " associated."] pub fn domain (& self) -> CFString { unsafe { let s = CFErrorGetDomain (self . 0) ; CFString :: wrap_under_get_rule (s) } } # [doc = " Returns the code identifying this type of error."] pub fn code (& self) -> CFIndex { unsafe { CFErrorGetCode (self . 0) } } # [doc = " Returns a human-presentable description of the error."] pub fn description (& self) -> CFString { unsafe { let s = CFErrorCopyDescription (self . 0) ; CFString :: wrap_under_create_rule (s) } } }
};
}
