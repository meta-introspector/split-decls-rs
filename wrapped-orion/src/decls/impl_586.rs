macro_rules! deps {
    () => {
        DHKEM_X25519_SHA256_CHACHA20!();
    };
}

macro_rules! impl_586 {
    () => {
        deps!();
        impl Psk for DHKEM_X25519_SHA256_CHACHA20 { }
    };
}

impl_586!()