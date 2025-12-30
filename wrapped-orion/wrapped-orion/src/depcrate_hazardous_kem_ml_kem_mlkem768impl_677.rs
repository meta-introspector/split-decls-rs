// Generated macro for impl_677 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem768impl_677 {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem768
// Provides: {"impl_677"}
// Dependencies: {}
impl EncapsulationKey { # [doc = " Instantiate a [EncapsulationKey] with key-checks from FIPS-203, section 7.2."] pub fn from_slice (slice : & [u8]) -> Result < Self , UnknownCryptoError > { Ok (Self { value : EncapKey :: < 3 , 1184 , MlKem768Internal > :: from_slice (slice) ? , }) } # [cfg (feature = "safe_api")] # [cfg_attr (docsrs , doc (cfg (feature = "safe_api")))] # [doc = " Given the [EncapsulationKey], generate a [SharedSecret] and associated [Ciphertext]."] pub fn encap (& self) -> Result < (SharedSecret , Ciphertext) , UnknownCryptoError > { use zeroize :: Zeroizing ; let mut m = Zeroizing :: new ([0u8 ; 32]) ; getrandom :: fill (m . as_mut ()) ? ; self . encap_deterministic (m . as_ref ()) } # [doc = " Given the [EncapsulationKey] and randomness `m`, generate a [SharedSecret] and associated [Ciphertext]."] pub fn encap_deterministic (& self , m : & [u8] ,) -> Result < (SharedSecret , Ciphertext) , UnknownCryptoError > { if m . len () != 32 { return Err (UnknownCryptoError) ; } let mut c = Ciphertext :: from_slice (& [0u8 ; MlKem768Internal :: CIPHERTEXT_SIZE]) ? ; let mut k_internal = self . value . mlkem_encap_internal (m . as_ref () , & mut c . value) ? ; let k = SharedSecret :: from_slice (k_internal . as_slice ()) ? ; k_internal . zeroize () ; Ok ((k , c)) } }
};
}
