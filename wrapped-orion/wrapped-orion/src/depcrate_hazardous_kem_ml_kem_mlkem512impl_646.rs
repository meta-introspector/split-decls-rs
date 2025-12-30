// Generated macro for impl_646 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem512impl_646 {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem512
// Provides: {"impl_646"}
// Dependencies: {}
impl TryFrom < & Seed > for KeyPair { type Error = UnknownCryptoError ; fn try_from (value : & Seed) -> Result < Self , Self :: Error > { let (ek , dk) = KeyPairInternal :: < MlKem512Internal > :: from_seed :: < 2 , 800 , 1632 > (value) ? ; Ok (Self { seed : Seed :: from_slice (value . unprotected_as_bytes ()) . unwrap () , dk : DecapsulationKey { value : dk , cached_ek : EncapsulationKey { value : ek } , } , }) } }
};
}
