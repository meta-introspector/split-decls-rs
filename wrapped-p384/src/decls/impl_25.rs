macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        # [cfg (feature = "sha384")] impl ecdsa_core :: hazmat :: DigestAlgorithm for NistP384 { type Digest = sha2 :: Sha384 ; }
    };
}

impl_25!()