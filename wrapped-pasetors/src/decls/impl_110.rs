macro_rules! deps {
    () => {
        V4!();
        Error!();
        AsymmetricPublicKey!();
        AsymmetricSecretKey!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl TryFrom < & AsymmetricSecretKey < V4 > > for AsymmetricPublicKey < V4 > { type Error = Error ; fn try_from (value : & AsymmetricSecretKey < V4 >) -> Result < Self , Self :: Error > { AsymmetricPublicKey :: < V4 > :: from (& value . as_bytes () [32 ..]) } }
    };
}

impl_110!();