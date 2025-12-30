// Generated macro for cancel_auto_extension (function)
macro_rules! Depcrate_auto_extensioncancel_auto_extension {
() => {
// Module: crate::auto_extension
// Provides: {"cancel_auto_extension"}
// Dependencies: {}
# [doc = " Unregister the initialization routine"] pub fn cancel_auto_extension (ax : RawAutoExtension) -> bool { unsafe { ffi :: sqlite3_cancel_auto_extension (Some (ax)) == 1 } }
};
}
