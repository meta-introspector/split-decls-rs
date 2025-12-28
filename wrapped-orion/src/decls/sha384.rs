macro_rules! deps {
    () => {
        UnknownCryptoError!();
        Sha384!();
    };
}

macro_rules! sha384 {
    () => {
        deps!();
        # [doc = " PBKDF2-HMAC-SHA384 (Password-Based Key Derivation Function 2) as specified in the [RFC 8018](https://tools.ietf.org/html/rfc8018)."] pub mod sha384 { use super :: * ; use crate :: hazardous :: hash :: sha2 :: sha384 :: { self , Sha384 } ; construct_hmac_key ! { # [doc = " A type to represent the `Password` that PBKDF2 hashes."] # [doc = ""] # [doc = " # Note:"] # [doc = " Because `Password` is used as a `SecretKey` for HMAC during hashing, `Password` already"] # [doc = " pads the given password to a length of 128, for use in HMAC, when initialized."] # [doc = ""] # [doc = " Using `unprotected_as_bytes()` will return the password with padding."] # [doc = ""] # [doc = " Using `get_length()` will return the length with padding (always 128)."] # [doc = ""] # [doc = " # Panics:"] # [doc = " A panic will occur if:"] # [doc = " - Failure to generate random bytes securely."] (Password , Sha384 , sha384 :: SHA384_OUTSIZE , test_pbkdf2_password , sha384 :: SHA384_BLOCKSIZE) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Derive a key using PBKDF2-HMAC-SHA384."] pub fn derive_key (password : & Password , salt : & [u8] , iterations : usize , dst_out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { _derive_key :: < hmac :: sha384 :: HmacSha384 , { sha384 :: SHA384_OUTSIZE } > (password . unprotected_as_bytes () , salt , iterations , dst_out ,) } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Verify PBKDF2-HMAC-SHA384 derived key in constant time."] pub fn verify (expected : & [u8] , password : & Password , salt : & [u8] , iterations : usize , dst_out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { _verify :: < hmac :: sha384 :: HmacSha384 , { sha384 :: SHA384_OUTSIZE } > (expected , password . unprotected_as_bytes () , salt , iterations , dst_out ,) } }
    };
}

sha384!()