// Generated macro for assure_ends_with_nl (function)
macro_rules! Depcrate_blob_builtin_driver_text_utilsassure_ends_with_nl {
() => {
// Module: crate::blob::builtin_driver::text::utils
// Provides: {"assure_ends_with_nl"}
// Dependencies: {}
pub fn assure_ends_with_nl (out : & mut Vec < u8 > , nl : & BStr) { if ! out . is_empty () && ! out . ends_with (b"\n") { out . push_str (nl) ; } }
};
}
