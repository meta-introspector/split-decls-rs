macro_rules! deps {
    () => {
        ChaCha20!();
        Poly1305!();
    };
}

macro_rules! poly1305_key_gen {
    () => {
        deps!();
        # [doc = " Poly1305 key generation using IETF ChaCha20."] pub (crate) fn poly1305_key_gen (ctx : & mut ChaCha20 , tmp_buffer : & mut Zeroizing < [u8 ; CHACHA_BLOCKSIZE] > ,) -> OneTimeKey { ctx . keystream_block (AUTH_CTR , tmp_buffer . as_mut ()) ; OneTimeKey :: from_slice (& tmp_buffer [.. POLY1305_KEYSIZE]) . unwrap () }
    };
}

poly1305_key_gen!()