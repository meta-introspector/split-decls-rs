macro_rules! deps {
    () => {
        DHKEM_X25519_SHA256_CHACHA20!();
    };
}

macro_rules! impl_581 {
    () => {
        deps!();
        impl PartialEq < DHKEM_X25519_SHA256_CHACHA20 > for DHKEM_X25519_SHA256_CHACHA20 { fn eq (& self , other : & DHKEM_X25519_SHA256_CHACHA20) -> bool { use subtle :: ConstantTimeEq ; (self . key . ct_eq (& other . key) & self . base_nonce . ct_eq (& other . base_nonce) & self . ctr . ct_eq (& other . ctr) & self . exporter_secret . ct_eq (& other . exporter_secret)) . into () } }
    };
}

impl_581!();