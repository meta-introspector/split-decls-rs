// Generated macro for xor (function)
macro_rules! Depcrate_block_apixor {
() => {
// Module: crate::block_api
// Provides: {"xor"}
// Dependencies: {}
# [inline (always)] fn xor < N : ArraySize > (buf : & mut Array < u8 , N > , data : & Array < u8 , N >) { for i in 0 .. N :: USIZE { buf [i] ^= data [i] ; } }
};
}
