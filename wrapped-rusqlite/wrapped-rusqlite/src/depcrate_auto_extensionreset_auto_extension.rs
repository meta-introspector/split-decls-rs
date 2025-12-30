// Generated macro for reset_auto_extension (function)
macro_rules! Depcrate_auto_extensionreset_auto_extension {
() => {
// Module: crate::auto_extension
// Provides: {"reset_auto_extension"}
// Dependencies: {}
# [doc = " Disable all automatic extensions previously registered"] pub fn reset_auto_extension () { unsafe { ffi :: sqlite3_reset_auto_extension () } }
};
}
