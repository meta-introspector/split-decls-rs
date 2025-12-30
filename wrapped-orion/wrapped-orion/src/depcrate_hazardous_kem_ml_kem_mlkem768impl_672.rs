// Generated macro for impl_672 (impl)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem768impl_672 {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem768
// Provides: {"impl_672"}
// Dependencies: {}
impl DecapsulationKey { # [doc = " Instantiate a [DecapsulationKey] with only key-checks from FIPS-203, section 7.3. Not MAL-BIND-K-CT secure."] pub fn unchecked_from_slice (slice : & [u8]) -> Result < Self , UnknownCryptoError > { let dk_unchecked = DecapKey :: < 3 , 1184 , 2400 , MlKem768Internal > :: unchecked_from_slice (slice) ? ; let ek_unchecked = EncapsulationKey :: from_slice (dk_unchecked . get_encapsulation_key_bytes ()) ? ; Ok (Self { value : dk_unchecked , cached_ek : ek_unchecked , }) } # [doc = " Perform decapsulation of a [Ciphertext]."] pub fn decap (& self , c : & Ciphertext) -> Result < SharedSecret , UnknownCryptoError > { let mut c_prime_buf = [0u8 ; MlKem768Internal :: CIPHERTEXT_SIZE] ; let mut k_internal = self . value . mlkem_decap_internal_with_ek (c . as_ref () , & mut c_prime_buf , & self . cached_ek . value ,) ? ; let k = SharedSecret :: from_slice (& k_internal) ? ; k_internal . zeroize () ; Ok (k) } }
};
}
