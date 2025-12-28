macro_rules! deps {
    () => {
        AsymmetricPublicKey!();
        AsymmetricSecretKey!();
        V2!();
        Error!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl TryFrom < & AsymmetricSecretKey < V2 > > for AsymmetricPublicKey < V2 > { type Error = Error ; fn try_from (value : & AsymmetricSecretKey < V2 >) -> Result < Self , Self :: Error > { AsymmetricPublicKey :: < V2 > :: from (& value . as_bytes () [32 ..]) } }
    };
}

impl_81!();