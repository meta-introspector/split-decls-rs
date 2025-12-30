// Generated macro for impl_652 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem512impl_652 {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem512
// Provides: {"impl_652"}
// Dependencies: {}
impl TryFrom < & DecapsulationKey > for EncapsulationKey { type Error = UnknownCryptoError ; fn try_from (value : & DecapsulationKey) -> Result < Self , Self :: Error > { Ok (Self { value : EncapKey :: < 2 , 800 , MlKem512Internal > :: from_slice (value . value . get_encapsulation_key_bytes () ,) ? , }) } }
};
}
