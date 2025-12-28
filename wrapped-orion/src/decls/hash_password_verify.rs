macro_rules! deps {
    () => {
        PasswordHash!();
        UnknownCryptoError!();
    };
}

macro_rules! hash_password_verify {
    () => {
        deps!();
        # [doc = " Hash and verify a password using Argon2i. The Argon2i parameters `iterations`"] # [doc = " and `memory` will be pulled from the `expected: &PasswordHash` argument. If"] # [doc = " you want to manually specify the iterations and memory for Argon2i to use in"] # [doc = " hashing the `password` argument, see the"] # [doc = " [`hazardous::kdf`](crate::hazardous::kdf::argon2i) module."] # [doc = ""] # [doc = " # Example:"] # [doc = " ```rust"] # [doc = " use orion::pwhash;"] # [doc = ""] # [doc = " let password = pwhash::Password::from_slice(b\"Secret password\")?;"] # [doc = " let wrong_password = pwhash::Password::from_slice(b\"hunter2\")?;"] # [doc = ""] # [doc = " // Pretend these are stored somewhere and out-of-mind, e.g. in a database."] # [doc = " let hash1 = pwhash::hash_password(&password, 3, 1<<15)?;"] # [doc = " let hash2 = pwhash::hash_password(&password, 4, 2<<15)?;"] # [doc = ""] # [doc = " // We don't have to remember which password used what parameters when it's"] # [doc = " // time to verify them. Both will correctly return `Ok(())`."] # [doc = " assert!(pwhash::hash_password_verify(&hash1, &password).is_ok());"] # [doc = " assert!(pwhash::hash_password_verify(&hash2, &password).is_ok());"] # [doc = ""] # [doc = " // The only way to get a failing result is to use the wrong password."] # [doc = " assert!(pwhash::hash_password_verify(&hash1, &wrong_password).is_err());"] # [doc = " # Ok::<(), orion::errors::UnknownCryptoError>(())"] # [doc = " ```"] # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] pub fn hash_password_verify (expected : & PasswordHash , password : & Password ,) -> Result < () , UnknownCryptoError > { let mut buffer = Zeroizing :: new ([0u8 ; PWHASH_LENGTH]) ; argon2i :: verify (expected . unprotected_as_bytes () , password . unprotected_as_bytes () , expected . salt . as_ref () , expected . iterations , expected . memory , None , None , buffer . as_mut () ,) }
    };
}

hash_password_verify!()