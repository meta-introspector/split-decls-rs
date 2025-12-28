macro_rules! deps {
    () => {
        AsymmetricSecretKey!();
        V3!();
        Error!();
        Generate!();
        AsymmetricPublicKey!();
        AsymmetricKeyPair!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl Generate < AsymmetricKeyPair < V3 > , V3 > for AsymmetricKeyPair < V3 > { fn generate () -> Result < AsymmetricKeyPair < V3 > , Error > { let key = SigningKey :: random (& mut OsRng) ; let public = AsymmetricPublicKey :: < V3 > :: from (VerifyingKey :: from (& key) . to_encoded_point (true) . as_ref () ,) ? ; let secret = AsymmetricSecretKey :: < V3 > :: from (key . to_bytes () . as_slice ()) ? ; Ok (Self { public , secret }) } }
    };
}

impl_95!();