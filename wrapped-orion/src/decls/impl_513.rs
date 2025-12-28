macro_rules! deps {
    () => {
        EncapsulationKey!();
        DecapsulationKey!();
        UnknownCryptoError!();
        MlKem768Internal!();
        EncapKey!();
    };
}

macro_rules! impl_513 {
    () => {
        deps!();
        impl TryFrom < & DecapsulationKey > for EncapsulationKey { type Error = UnknownCryptoError ; fn try_from (value : & DecapsulationKey) -> Result < Self , Self :: Error > { Ok (Self { value : EncapKey :: < 3 , 1184 , MlKem768Internal > :: from_slice (value . value . get_encapsulation_key_bytes () ,) ? , }) } }
    };
}

impl_513!();