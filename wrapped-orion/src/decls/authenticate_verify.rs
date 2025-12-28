macro_rules! deps {
    () => {
        Blake2b!();
        UnknownCryptoError!();
    };
}

macro_rules! authenticate_verify {
    () => {
        deps!();
        # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Authenticate and verify a message using BLAKE2b-256 in keyed mode."] pub fn authenticate_verify (expected : & Tag , secret_key : & SecretKey , data : & [u8] ,) -> Result < () , UnknownCryptoError > { if secret_key . len () < BLAKE2B_MIN_KEY_SIZE || expected . len () != BLAKE2B_TAG_SIZE { return Err (UnknownCryptoError) ; } let key = blake2b :: SecretKey :: from_slice (secret_key . unprotected_as_bytes ()) ? ; Blake2b :: verify (expected , & key , BLAKE2B_TAG_SIZE , data) }
    };
}

authenticate_verify!();