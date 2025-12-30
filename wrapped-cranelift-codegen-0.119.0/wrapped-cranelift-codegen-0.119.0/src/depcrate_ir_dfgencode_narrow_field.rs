// Generated macro for encode_narrow_field (function)
macro_rules! Depcrate_ir_dfgencode_narrow_field {
() => {
// Module: crate::ir::dfg
// Provides: {"encode_narrow_field"}
// Dependencies: {}
# [doc = " Encodes a value in 0..2^32 into 0..2^n, where n is less than 32"] # [doc = " (and is implied by `mask`), by translating 2^32-1 (0xffffffff)"] # [doc = " into 2^n-1 and panic'ing on 2^n..2^32-1."] fn encode_narrow_field (x : u32 , bits : u8) -> u32 { let max = (1 << bits) - 1 ; if x == 0xffff_ffff { max } else { debug_assert ! (x < max , "{x} does not fit into {bits} bits (must be less than {max} to \
             allow for a 0xffffffff sentinel)") ; x } }
};
}
