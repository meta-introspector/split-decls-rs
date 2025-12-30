// Generated macro for undo_bytes (function)
macro_rules! Depcrate_inflate_coreundo_bytes {
() => {
// Module: crate::inflate::core
// Provides: {"undo_bytes"}
// Dependencies: {}
# [inline] fn undo_bytes (l : & mut LocalVars , max : u32) -> u32 { let res = cmp :: min (l . num_bits >> 3 , max) ; l . num_bits -= res << 3 ; res }
};
}
