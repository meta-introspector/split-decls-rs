macro_rules! deps {
    () => {
        Generate!();
        AsymmetricPublicKey!();
        AsymmetricKeyPair!();
        V4!();
        Error!();
        AsymmetricSecretKey!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl Generate < AsymmetricKeyPair < V4 > , V4 > for AsymmetricKeyPair < V4 > { fn generate () -> Result < AsymmetricKeyPair < V4 > , Error > { let key_pair = KeyPair :: generate () ; let secret = AsymmetricSecretKey :: < V4 > :: from (key_pair . sk . as_ref ()) . map_err (| _ | Error :: KeyGeneration) ? ; let public = AsymmetricPublicKey :: < V4 > :: from (key_pair . pk . as_ref ()) . map_err (| _ | Error :: KeyGeneration) ? ; Ok (Self { public , secret }) } }
    };
}

impl_111!();