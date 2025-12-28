macro_rules! deps {
    () => {
        PasswordHash!();
        UnknownCryptoError!();
    };
}

macro_rules! hash_password {
    () => {
        deps!();
        # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Hash a password using Argon2i."] pub fn hash_password (password : & Password , iterations : u32 , memory : u32 ,) -> Result < PasswordHash , UnknownCryptoError > { if iterations < MIN_ITERATIONS { return Err (UnknownCryptoError) ; } let salt = Salt :: generate (SALT_LENGTH) . unwrap () ; let mut buffer = Zeroizing :: new ([0u8 ; PWHASH_LENGTH]) ; argon2i :: derive_key (password . unprotected_as_bytes () , salt . as_ref () , iterations , memory , None , None , buffer . as_mut () ,) ? ; PasswordHash :: from_slice (buffer . as_ref () , salt . as_ref () , iterations , memory) }
    };
}

hash_password!();