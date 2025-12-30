// Generated macro for impl_669 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem768impl_669 {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem768
// Provides: {"impl_669"}
// Dependencies: {}
impl TryFrom < & Seed > for KeyPair { type Error = UnknownCryptoError ; fn try_from (value : & Seed) -> Result < Self , Self :: Error > { let (ek , dk) = KeyPairInternal :: < MlKem768Internal > :: from_seed :: < 3 , 1184 , 2400 > (value) ? ; Ok (Self { seed : Seed :: from_slice (value . unprotected_as_bytes ()) . unwrap () , dk : DecapsulationKey { value : dk , cached_ek : EncapsulationKey { value : ek } , } , }) } }
};
}
