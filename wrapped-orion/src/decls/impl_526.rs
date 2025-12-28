macro_rules! deps {
    () => {
        MlKem1024Internal!();
        KeyPairInternal!();
        KeyPair!();
        UnknownCryptoError!();
        DecapsulationKey!();
        EncapsulationKey!();
    };
}

macro_rules! impl_526 {
    () => {
        deps!();
        impl TryFrom < & Seed > for KeyPair { type Error = UnknownCryptoError ; fn try_from (value : & Seed) -> Result < Self , Self :: Error > { let (ek , dk) = KeyPairInternal :: < MlKem1024Internal > :: from_seed :: < 4 , 1568 , 3168 > (value) ? ; Ok (Self { seed : Seed :: from_slice (value . unprotected_as_bytes ()) . unwrap () , dk : DecapsulationKey { value : dk , cached_ek : EncapsulationKey { value : ek } , } , }) } }
    };
}

impl_526!()