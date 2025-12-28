macro_rules! deps {
    () => {
        PublicKey!();
        FieldElement!();
        PrivateKey!();
        Scalar!();
        UnknownCryptoError!();
    };
}

macro_rules! impl_383 {
    () => {
        deps!();
        impl TryFrom < & PrivateKey > for PublicKey { type Error = UnknownCryptoError ; fn try_from (private_key : & PrivateKey) -> Result < Self , Self :: Error > { let scalar = Scalar :: from_slice (private_key . unprotected_as_bytes ()) ? ; Ok (PublicKey :: from (mont_ladder (& scalar , FieldElement :: from_bytes (& BASEPOINT)) . as_bytes () ,)) } }
    };
}

impl_383!();