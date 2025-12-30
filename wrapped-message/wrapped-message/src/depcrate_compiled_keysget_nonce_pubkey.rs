// Generated macro for get_nonce_pubkey (function)
macro_rules! Depcrate_compiled_keysget_nonce_pubkey {
() => {
// Module: crate::compiled_keys
// Provides: {"get_nonce_pubkey"}
// Dependencies: {}
fn get_nonce_pubkey (instructions : & [Instruction]) -> Option < & Address > { let ix = instructions . get (NONCED_TX_MARKER_IX_INDEX) ? ; if ! system_program :: check_id (& ix . program_id) { return None ; } if ! is_advance_nonce_instruction_data (& ix . data) { return None ; } ix . accounts . first () . map (| meta | & meta . pubkey) }
};
}
