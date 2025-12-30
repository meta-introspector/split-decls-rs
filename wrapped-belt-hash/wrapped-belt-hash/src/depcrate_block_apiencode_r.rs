// Generated macro for encode_r (function)
macro_rules! Depcrate_block_apiencode_r {
() => {
// Module: crate::block_api
// Provides: {"encode_r"}
// Dependencies: {}
# [inline (always)] fn encode_r (r : u128) -> [u32 ; 4] { core :: array :: from_fn (| i | (r >> (32 * i)) as u32) }
};
}
