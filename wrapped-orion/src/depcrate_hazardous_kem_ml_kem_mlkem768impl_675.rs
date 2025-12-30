// Generated macro for impl_675 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem768impl_675 {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem768
// Provides: {"impl_675"}
// Dependencies: {}
impl TryFrom < & DecapsulationKey > for EncapsulationKey { type Error = UnknownCryptoError ; fn try_from (value : & DecapsulationKey) -> Result < Self , Self :: Error > { Ok (Self { value : EncapKey :: < 3 , 1184 , MlKem768Internal > :: from_slice (value . value . get_encapsulation_key_bytes () ,) ? , }) } }
};
}
