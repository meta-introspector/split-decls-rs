// Generated macro for xor_keystream (function)
macro_rules! Depcrate_hazardous_stream_chacha20xor_keystream {
() => {
// Module: crate::hazardous::stream::chacha20
// Provides: {"xor_keystream"}
// Dependencies: {}
# [doc = " XOR keystream into destination array using a temporary buffer for each keystream block."] pub (crate) fn xor_keystream (ctx : & mut ChaCha20 , initial_counter : u32 , tmp_block : & mut [u8] , bytes : & mut [u8] ,) -> Result < () , UnknownCryptoError > { debug_assert_eq ! (tmp_block . len () , CHACHA_BLOCKSIZE) ; if bytes . is_empty () { return Err (UnknownCryptoError) ; } for (ctr , out_block) in bytes . chunks_mut (CHACHA_BLOCKSIZE) . enumerate () { match initial_counter . checked_add (ctr as u32) { Some (counter) => { ctx . keystream_block (counter , tmp_block) ; xor_slices ! (tmp_block , out_block) ; } None => return Err (UnknownCryptoError) , } } Ok (()) }
};
}
