// Generated macro for impl_698 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem1024impl_698 {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem1024
// Provides: {"impl_698"}
// Dependencies: {}
impl TryFrom < & DecapsulationKey > for EncapsulationKey { type Error = UnknownCryptoError ; fn try_from (value : & DecapsulationKey) -> Result < Self , Self :: Error > { Ok (Self { value : EncapKey :: < 4 , 1568 , MlKem1024Internal > :: from_slice (value . value . get_encapsulation_key_bytes () ,) ? , }) } }
};
}
