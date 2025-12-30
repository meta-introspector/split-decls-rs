// Generated macro for is_advance_nonce_instruction_data (function)
macro_rules! Depcrate_inline_nonceis_advance_nonce_instruction_data {
() => {
// Module: crate::inline_nonce
// Provides: {"is_advance_nonce_instruction_data"}
// Dependencies: {}
# [doc = " Check if the given instruction data is the same as"] # [doc = " `SystemInstruction::AdvanceNonceAccount`."] # [doc = ""] # [doc = " NOTE: It's possible for additional data to exist after the 4th byte, but"] # [doc = " users of this function only look at the first 4 bytes."] pub fn is_advance_nonce_instruction_data (data : & [u8]) -> bool { data . get (0 .. 4) == Some (& ADVANCE_NONCE_DATA) }
};
}
