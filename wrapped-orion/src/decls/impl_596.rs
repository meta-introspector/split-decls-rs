macro_rules! deps {
    () => {
        PublicKey!();
    };
}

macro_rules! impl_596 {
    () => {
        deps!();
        impl HpkePublicKey for crate :: hazardous :: kem :: x25519_hkdf_sha256 :: PublicKey { }
    };
}

impl_596!()