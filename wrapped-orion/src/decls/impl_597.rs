macro_rules! deps {
    () => {
        PublicKey!();
    };
}

macro_rules! impl_597 {
    () => {
        deps!();
        impl HpkeEncapKey for crate :: hazardous :: kem :: x25519_hkdf_sha256 :: PublicKey { }
    };
}

impl_597!()