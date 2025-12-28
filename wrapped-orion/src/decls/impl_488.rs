macro_rules! deps {
    () => {
        KeyPair!();
        MlKem512Internal!();
        EncapsulationKey!();
        UnknownCryptoError!();
        KeyPairInternal!();
        DecapsulationKey!();
    };
}

macro_rules! impl_488 {
    () => {
        deps!();
        impl TryFrom < & Seed > for KeyPair { type Error = UnknownCryptoError ; fn try_from (value : & Seed) -> Result < Self , Self :: Error > { let (ek , dk) = KeyPairInternal :: < MlKem512Internal > :: from_seed :: < 2 , 800 , 1632 > (value) ? ; Ok (Self { seed : Seed :: from_slice (value . unprotected_as_bytes ()) . unwrap () , dk : DecapsulationKey { value : dk , cached_ek : EncapsulationKey { value : ek } , } , }) } }
    };
}

impl_488!()