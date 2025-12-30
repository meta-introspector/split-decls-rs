// Generated macro for write_tokens (function)
macro_rules! Depcrate_blob_builtin_driver_text_utilswrite_tokens {
() => {
// Module: crate::blob::builtin_driver::text::utils
// Provides: {"write_tokens"}
// Dependencies: {}
fn write_tokens (interner : & Interner < & [u8] > , tokens : & [Token] , out : & mut Vec < u8 > ,) { for token in tokens { out . extend_from_slice (interner [* token]) ; } }
};
}
