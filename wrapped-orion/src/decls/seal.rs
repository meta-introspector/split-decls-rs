macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! seal {
    () => {
        deps!();
        # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Authenticated encryption using XChaCha20Poly1305."] pub fn seal (secret_key : & SecretKey , plaintext : & [u8]) -> Result < Vec < u8 > , UnknownCryptoError > { if plaintext . is_empty () { return Err (UnknownCryptoError) ; } let out_len = match plaintext . len () . checked_add (XCHACHA_NONCESIZE + POLY1305_OUTSIZE) { Some (min_out_len) => min_out_len , None => return Err (UnknownCryptoError) , } ; let mut dst_out = vec ! [0u8 ; out_len] ; let nonce = Nonce :: generate () ; dst_out [.. XCHACHA_NONCESIZE] . copy_from_slice (nonce . as_ref ()) ; aead :: xchacha20poly1305 :: seal (& chacha20 :: SecretKey :: from_slice (secret_key . unprotected_as_bytes ()) ? , & nonce , plaintext , None , & mut dst_out [XCHACHA_NONCESIZE ..] ,) ? ; Ok (dst_out) }
    };
}

seal!()