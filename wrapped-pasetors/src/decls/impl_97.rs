macro_rules! deps {
    () => {
        UncompressedPublicKey!();
        Error!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl TryFrom < & [u8] > for UncompressedPublicKey { type Error = Error ; fn try_from (value : & [u8]) -> Result < Self , Self :: Error > { if value . len () != 97 && value [0] != 4 { return Err (Error :: Key) ; } let pk = PublicKey :: from_sec1_bytes (value) . map_err (| _ | Error :: Key) ? ; Ok (Self (pk)) } }
    };
}

impl_97!();