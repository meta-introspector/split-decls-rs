// Generated macro for impl_692 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem1024impl_692 {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem1024
// Provides: {"impl_692"}
// Dependencies: {}
impl TryFrom < & Seed > for KeyPair { type Error = UnknownCryptoError ; fn try_from (value : & Seed) -> Result < Self , Self :: Error > { let (ek , dk) = KeyPairInternal :: < MlKem1024Internal > :: from_seed :: < 4 , 1568 , 3168 > (value) ? ; Ok (Self { seed : Seed :: from_slice (value . unprotected_as_bytes ()) . unwrap () , dk : DecapsulationKey { value : dk , cached_ek : EncapsulationKey { value : ek } , } , }) } }
};
}
