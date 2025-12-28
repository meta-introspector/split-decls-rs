macro_rules! deps {
    () => {
        ChaCha20!();
        UnknownCryptoError!();
    };
}

macro_rules! hchacha20 {
    () => {
        deps!();
        # [doc = " HChaCha20 as specified in the [draft-RFC](https://github.com/bikeshedders/xchacha-rfc/blob/master)."] pub (super) fn hchacha20 (secret_key : & SecretKey , nonce : & [u8] ,) -> Result < [u8 ; HCHACHA_OUTSIZE] , UnknownCryptoError > { let mut chacha_state = ChaCha20 :: new (secret_key . unprotected_as_bytes () , nonce , false) ? ; let mut keystream_block = [0u8 ; HCHACHA_OUTSIZE] ; chacha_state . keystream_block (0 , & mut keystream_block) ; Ok (keystream_block) }
    };
}

hchacha20!()