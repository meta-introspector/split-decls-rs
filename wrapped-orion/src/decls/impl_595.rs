macro_rules! deps {
    () => {
        PrivateKey!();
    };
}

macro_rules! impl_595 {
    () => {
        deps!();
        impl HpkePrivateKey for crate :: hazardous :: kem :: x25519_hkdf_sha256 :: PrivateKey { }
    };
}

impl_595!()