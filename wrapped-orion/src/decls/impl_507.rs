macro_rules! deps {
    () => {
        UnknownCryptoError!();
        KeyPairInternal!();
        MlKem768Internal!();
        KeyPair!();
        DecapsulationKey!();
        EncapsulationKey!();
    };
}

macro_rules! impl_507 {
    () => {
        deps!();
        impl TryFrom < & Seed > for KeyPair { type Error = UnknownCryptoError ; fn try_from (value : & Seed) -> Result < Self , Self :: Error > { let (ek , dk) = KeyPairInternal :: < MlKem768Internal > :: from_seed :: < 3 , 1184 , 2400 > (value) ? ; Ok (Self { seed : Seed :: from_slice (value . unprotected_as_bytes ()) . unwrap () , dk : DecapsulationKey { value : dk , cached_ek : EncapsulationKey { value : ek } , } , }) } }
    };
}

impl_507!();