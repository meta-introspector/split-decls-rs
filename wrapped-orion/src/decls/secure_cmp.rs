macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! secure_cmp {
    () => {
        deps!();
        # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Compare two equal length slices in constant time."] # [doc = ""] # [doc = " # About:"] # [doc = " Compare two equal length slices, in constant time, using the"] # [doc = " [subtle](https://github.com/dalek-cryptography/subtle) crate."] # [doc = ""] # [doc = " # Parameters:"] # [doc = " - `a`: The first slice used in the comparison."] # [doc = " - `b`: The second slice used in the comparison."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `a` and `b` do not have the same length."] # [doc = " - `a` is not equal to `b`."] # [doc = ""] # [doc = " # Example:"] # [doc = " ```rust"] # [doc = " # #[cfg(feature = \"safe_api\")] {"] # [doc = " use orion::util;"] # [doc = ""] # [doc = " let mut rnd_bytes = [0u8; 64];"] # [doc = " assert!(util::secure_cmp(&rnd_bytes, &[0u8; 64]).is_ok());"] # [doc = ""] # [doc = " util::secure_rand_bytes(&mut rnd_bytes)?;"] # [doc = " assert!(util::secure_cmp(&rnd_bytes, &[0u8; 64]).is_err());"] # [doc = " # }"] # [doc = " # Ok::<(), orion::errors::UnknownCryptoError>(())"] # [doc = " ```"] pub fn secure_cmp (a : & [u8] , b : & [u8]) -> Result < () , errors :: UnknownCryptoError > { if a . ct_eq (b) . into () { Ok (()) } else { Err (errors :: UnknownCryptoError) } }
    };
}

secure_cmp!()