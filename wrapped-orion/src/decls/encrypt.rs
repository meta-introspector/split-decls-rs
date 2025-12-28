macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! encrypt {
    () => {
        deps!();
        # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " XChaCha20 encryption as specified in the [draft RFC](https://tools.ietf.org/html/draft-irtf-cfrg-xchacha-03)."] pub fn encrypt (secret_key : & SecretKey , nonce : & Nonce , initial_counter : u32 , plaintext : & [u8] , dst_out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { let (subkey , ietf_nonce) = subkey_and_nonce (secret_key , nonce) ; chacha20 :: encrypt (& subkey , & ietf_nonce , initial_counter , plaintext , dst_out) }
    };
}

encrypt!();