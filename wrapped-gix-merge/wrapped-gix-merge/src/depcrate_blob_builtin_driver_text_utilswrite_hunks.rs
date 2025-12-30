// Generated macro for write_hunks (function)
macro_rules! Depcrate_blob_builtin_driver_text_utilswrite_hunks {
() => {
// Module: crate::blob::builtin_driver::text::utils
// Provides: {"write_hunks"}
// Dependencies: {}
pub fn write_hunks (hunks : & [Hunk] , input : & InternedInput < & [u8] > , current_tokens : & [Token] , out : & mut Vec < u8 > ,) { for hunk in hunks { let (tokens , range) = match hunk . side { Side :: Current => (current_tokens , & hunk . after) , Side :: Other => (input . after . as_slice () , & hunk . after) , Side :: Ancestor => (input . before . as_slice () , & hunk . before) , } ; write_tokens (& input . interner , & tokens [usize_range (range)] , out) ; } }
};
}
