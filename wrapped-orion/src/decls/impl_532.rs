macro_rules! deps {
    () => {
        UnknownCryptoError!();
        EncapKey!();
        MlKem1024Internal!();
        DecapsulationKey!();
        EncapsulationKey!();
    };
}

macro_rules! impl_532 {
    () => {
        deps!();
        impl TryFrom < & DecapsulationKey > for EncapsulationKey { type Error = UnknownCryptoError ; fn try_from (value : & DecapsulationKey) -> Result < Self , Self :: Error > { Ok (Self { value : EncapKey :: < 4 , 1568 , MlKem1024Internal > :: from_slice (value . value . get_encapsulation_key_bytes () ,) ? , }) } }
    };
}

impl_532!()