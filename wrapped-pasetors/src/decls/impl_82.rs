macro_rules! deps {
    () => {
        AsymmetricKeyPair!();
        Generate!();
        AsymmetricSecretKey!();
        Error!();
        AsymmetricPublicKey!();
        V2!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl Generate < AsymmetricKeyPair < V2 > , V2 > for AsymmetricKeyPair < V2 > { fn generate () -> Result < AsymmetricKeyPair < V2 > , Error > { let key_pair = KeyPair :: generate () ; let secret = AsymmetricSecretKey :: < V2 > :: from (key_pair . sk . as_ref ()) . map_err (| _ | Error :: KeyGeneration) ? ; let public = AsymmetricPublicKey :: < V2 > :: from (key_pair . pk . as_ref ()) . map_err (| _ | Error :: KeyGeneration) ? ; Ok (Self { public , secret }) } }
    };
}

impl_82!()