macro_rules! deps {
    () => {
        DHKEM_X25519_SHA256_CHACHA20!();
    };
}

macro_rules! impl_584 {
    () => {
        deps!();
        impl Drop for DHKEM_X25519_SHA256_CHACHA20 { fn drop (& mut self) { use zeroize :: Zeroize ; self . key . iter_mut () . zeroize () ; self . exporter_secret . iter_mut () . zeroize () ; } }
    };
}

impl_584!()