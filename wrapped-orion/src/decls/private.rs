macro_rules! private {
    () => {
        pub (crate) mod private { # [doc = " Marker trait for a public key, for a corresponding HPKE private key."] pub trait HpkePublicKey { } # [doc = " Marker trait for an encapsulated key, that is generated with HPKE."] pub trait HpkeEncapKey { } # [doc = " Marker trait for a private key, that can be used with HPKE."] pub trait HpkePrivateKey { } }
    };
}

private!()