macro_rules! deps {
    () => {
        DHKEM_X25519_SHA256_CHACHA20!();
    };
}

macro_rules! impl_583 {
    () => {
        deps!();
        impl core :: fmt :: Debug for DHKEM_X25519_SHA256_CHACHA20 { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{} key: {{***OMITTED***}}, base_nonce: {:?}, ctr: {:?}, exporter_secret: {{***OMITTED***}}" , stringify ! (DHKEM_X25519_SHA256_CHACHA20) , & self . base_nonce , self . ctr) } }
    };
}

impl_583!()