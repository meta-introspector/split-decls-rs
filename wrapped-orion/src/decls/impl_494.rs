macro_rules! deps {
    () => {
        MlKem512Internal!();
        UnknownCryptoError!();
        EncapKey!();
        DecapsulationKey!();
        EncapsulationKey!();
    };
}

macro_rules! impl_494 {
    () => {
        deps!();
        impl TryFrom < & DecapsulationKey > for EncapsulationKey { type Error = UnknownCryptoError ; fn try_from (value : & DecapsulationKey) -> Result < Self , Self :: Error > { Ok (Self { value : EncapKey :: < 2 , 800 , MlKem512Internal > :: from_slice (value . value . get_encapsulation_key_bytes () ,) ? , }) } }
    };
}

impl_494!();