// Generated macro for decode_narrow_field (function)
macro_rules! Depcrate_ir_dfgdecode_narrow_field {
() => {
// Module: crate::ir::dfg
// Provides: {"decode_narrow_field"}
// Dependencies: {}
# [doc = " The inverse of the above `encode_narrow_field`: unpacks 2^n-1 into"] # [doc = " 2^32-1."] fn decode_narrow_field (x : u32 , bits : u8) -> u32 { if x == (1 << bits) - 1 { 0xffff_ffff } else { x } }
};
}
