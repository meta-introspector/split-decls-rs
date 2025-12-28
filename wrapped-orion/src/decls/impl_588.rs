macro_rules! deps {
    () => {
        DHKEM_X25519_SHA256_CHACHA20!();
    };
}

macro_rules! impl_588 {
    () => {
        deps!();
        impl AuthPsk for DHKEM_X25519_SHA256_CHACHA20 { }
    };
}

impl_588!();