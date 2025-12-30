// Generated macro for impl_173 (impl)
macro_rules! Depcrate_crypto_providerimpl_173 {
() => {
// Module: crate::crypto_provider
// Provides: {"impl_173"}
// Dependencies: {}
impl Hpke { # [doc = " Chooses a random supported HPKE suite and generates a throw-away public key."] # [doc = ""] # [doc = " Returns both the selected Rustls `hpke::Hpke` suite and the `hpke::HpkePublicKey`"] # [doc = " or `None` if an error occurs."] pub (crate) fn grease_public_key (& self , provider : & CryptoProvider ,) -> Option < (& dyn hpke :: Hpke , hpke :: HpkePublicKey) > { let num_suites = self . suites . len () ; if num_suites == 0 { return None ; } debug_assert ! (num_suites < u32 :: MAX as usize) ; let mut buf = [0u8 ; 4] ; let threshold = u32 :: MAX - (u32 :: MAX % num_suites as u32) ; let suite = loop { provider . secure_random . fill (& mut buf) . ok () ? ; let value = u32 :: from_ne_bytes (buf) ; if value < threshold { break self . suites [value as usize / (threshold as usize / num_suites)] ; } } ; let pk = suite . generate_key_pair () . map (| pair | pair . 0) . ok () ? ; Some ((suite , pk)) } }
};
}
