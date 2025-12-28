macro_rules! deps {
    () => {
        AsymmetricPublicKey!();
        V3!();
        Error!();
        UncompressedPublicKey!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl TryFrom < & AsymmetricPublicKey < V3 > > for UncompressedPublicKey { type Error = Error ; fn try_from (value : & AsymmetricPublicKey < V3 >) -> Result < Self , Self :: Error > { if value . as_bytes () [0] != 2 && value . as_bytes () [0] != 3 { return Err (Error :: Key) ; } let pk = PublicKey :: from_sec1_bytes (value . as_bytes ()) . map_err (| _ | Error :: Key) ? ; Ok (UncompressedPublicKey (pk)) } }
    };
}

impl_98!()