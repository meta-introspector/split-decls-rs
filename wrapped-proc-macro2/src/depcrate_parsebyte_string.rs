// Generated macro for byte_string (function)
macro_rules! Depcrate_parsebyte_string {
() => {
// Module: crate::parse
// Provides: {"byte_string"}
// Dependencies: {}
fn byte_string (input : Cursor) -> Result < Cursor , Reject > { if let Ok (input) = input . parse ("b\"") { cooked_byte_string (input) } else if let Ok (input) = input . parse ("br") { raw_byte_string (input) } else { Err (Reject) } }
};
}
