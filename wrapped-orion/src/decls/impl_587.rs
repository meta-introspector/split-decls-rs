macro_rules! deps {
    () => {
        DHKEM_X25519_SHA256_CHACHA20!();
    };
}

macro_rules! impl_587 {
    () => {
        deps!();
        impl Auth for DHKEM_X25519_SHA256_CHACHA20 { }
    };
}

impl_587!();