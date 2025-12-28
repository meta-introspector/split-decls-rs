macro_rules! deps {
    () => {
        V3!();
        Error!();
        AsymmetricPublicKey!();
        AsymmetricSecretKey!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl TryFrom < & AsymmetricSecretKey < V3 > > for AsymmetricPublicKey < V3 > { type Error = Error ; fn try_from (value : & AsymmetricSecretKey < V3 >) -> Result < Self , Self :: Error > { let sk = SigningKey :: from_bytes (value . as_bytes () . into ()) . map_err (| _ | Error :: Key) ? ; AsymmetricPublicKey :: < V3 > :: from (sk . verifying_key () . to_encoded_point (true) . as_bytes ()) } }
    };
}

impl_94!();