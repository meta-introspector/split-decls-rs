macro_rules! deps {
    () => {
        UnknownCryptoError!();
        ChaCha20!();
    };
}

macro_rules! encrypt_in_place {
    () => {
        deps!();
        # [doc = " In-place IETF ChaCha20 encryption as specified in the [RFC 8439](https://tools.ietf.org/html/rfc8439)."] pub (crate) fn encrypt_in_place (secret_key : & SecretKey , nonce : & Nonce , initial_counter : u32 , bytes : & mut [u8] ,) -> Result < () , UnknownCryptoError > { if bytes . is_empty () { return Err (UnknownCryptoError) ; } let mut ctx = ChaCha20 :: new (secret_key . unprotected_as_bytes () , nonce . as_ref () , true) ? ; let mut keystream_block = Zeroizing :: new ([0u8 ; CHACHA_BLOCKSIZE]) ; xor_keystream (& mut ctx , initial_counter , keystream_block . as_mut () , bytes) }
    };
}

encrypt_in_place!()