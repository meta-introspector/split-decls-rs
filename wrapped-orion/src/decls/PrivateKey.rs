macro_rules! deps {
    () => {
        Scalar!();
    };
}

macro_rules! PrivateKey {
    () => {
        deps!();
        # [allow (clippy :: derive_partial_eq_without_eq)] # [doc = " A type to represent the `PrivateKey` that X25519 uses."] # [doc = ""] # [doc = " This type holds a scalar and is used internally as such. The scalar held is decoded"] # [doc = " (a.k.a \"clamped\") as mandated in the [RFC](https://datatracker.ietf.org/doc/html/rfc7748#section-5)."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] # [doc = ""] # [doc = " # Panics:"] # [doc = " A panic will occur if:"] # [doc = " - Failure to generate random bytes securely."] # [doc = ""] # [doc = ""] # [doc = " # Security:"] # [doc = " - __**Avoid using**__ `unprotected_as_bytes()` whenever possible, as it breaks all protections"] # [doc = "   that the type implements."] # [doc = ""] # [doc = " - The trait `PartialEq<&'_ [u8]>` is implemented for this type so that users are not tempted"] # [doc = "   to call `unprotected_as_bytes` to compare this sensitive value to a byte slice. The trait"] # [doc = "   is implemented in such a way that the comparison happens in constant time. Thus, users should"] # [doc = "   prefer `SecretType == &[u8]` over `SecretType.unprotected_as_bytes() == &[u8]`."] # [doc = ""] # [doc = " Examples are shown below. The examples apply to any type that implements `PartialEq<&'_ [u8]>`."] # [doc = " ```rust"] # [doc = " # #[cfg(feature = \"safe_api\")] {"] # [doc = " use orion::hazardous::ecc::x25519::PrivateKey;"] # [doc = ""] # [doc = " // Initialize a secret key with random bytes."] # [doc = " let secret_key = PrivateKey::generate();"] # [doc = ""] # [doc = " // Secure, constant-time comparison with a byte slice"] # [doc = " assert_ne!(secret_key, &[0; 32][..]);"] # [doc = ""] # [doc = " // Secure, constant-time comparison with another SecretKey"] # [doc = " assert_ne!(secret_key, PrivateKey::generate());"] # [doc = " # }"] # [doc = " # Ok::<(), orion::errors::UnknownCryptoError>(())"] # [doc = " ```"] # [derive (PartialEq)] pub struct PrivateKey { scalar : Scalar , }
    };
}

PrivateKey!();