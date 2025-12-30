// Generated macro for encrypt (function)
macro_rules! Depcrate_hazardous_stream_chacha20encrypt {
() => {
// Module: crate::hazardous::stream::chacha20
// Provides: {"encrypt"}
// Dependencies: {}
# [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " IETF ChaCha20 encryption as specified in the [RFC 8439](https://tools.ietf.org/html/rfc8439)."] pub fn encrypt (secret_key : & SecretKey , nonce : & Nonce , initial_counter : u32 , plaintext : & [u8] , dst_out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { if dst_out . len () < plaintext . len () { return Err (UnknownCryptoError) ; } if plaintext . is_empty () { return Err (UnknownCryptoError) ; } let mut ctx = ChaCha20 :: new (secret_key . unprotected_as_bytes () , nonce . as_ref () , true) ? ; let mut keystream_block = Zeroizing :: new ([0u8 ; CHACHA_BLOCKSIZE]) ; for (ctr , (p_block , c_block)) in plaintext . chunks (CHACHA_BLOCKSIZE) . zip (dst_out . chunks_mut (CHACHA_BLOCKSIZE)) . enumerate () { match initial_counter . checked_add (ctr as u32) { Some (counter) => { ctx . next_produceable () ? ; ctx . keystream_block (counter , keystream_block . as_mut ()) ; xor_slices ! (p_block , keystream_block . as_mut ()) ; c_block [.. p_block . len ()] . copy_from_slice (& keystream_block . as_ref () [.. p_block . len ()]) ; } None => return Err (UnknownCryptoError) , } } Ok (()) }
};
}
