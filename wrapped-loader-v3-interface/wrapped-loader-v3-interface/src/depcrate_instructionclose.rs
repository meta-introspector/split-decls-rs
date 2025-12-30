// Generated macro for close (function)
macro_rules! Depcrate_instructionclose {
() => {
// Module: crate::instruction
// Provides: {"close"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Returns the instructions required to close a buffer account"] pub fn close (close_address : & Pubkey , recipient_address : & Pubkey , authority_address : & Pubkey ,) -> Instruction { close_any (close_address , recipient_address , Some (authority_address) , None ,) }
};
}
