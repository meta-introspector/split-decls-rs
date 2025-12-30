// Generated macro for RawAutoExtension (type)
macro_rules! Depcrate_auto_extensionRawAutoExtension {
() => {
// Module: crate::auto_extension
// Provides: {"RawAutoExtension"}
// Dependencies: {}
# [doc = " Raw automatic extension initialization routine"] pub type RawAutoExtension = unsafe extern "C" fn (db : * mut ffi :: sqlite3 , pz_err_msg : * mut * mut c_char , _ : * const ffi :: sqlite3_api_routines ,) -> c_int ;
};
}
