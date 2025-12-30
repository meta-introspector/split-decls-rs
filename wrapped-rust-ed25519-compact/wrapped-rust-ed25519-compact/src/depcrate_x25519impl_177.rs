// Generated macro for impl_177 (impl)
macro_rules! Depcrate_x25519impl_177 {
() => {
// Module: crate::x25519
// Provides: {"impl_177"}
// Dependencies: {}
impl KeyPair { # [doc = " Generates a new key pair."] # [cfg (feature = "random")] pub fn generate () -> KeyPair { let mut sk = [0u8 ; SecretKey :: BYTES] ; getrandom :: getrandom (& mut sk) . expect ("getrandom") ; if Fe :: from_bytes (& sk) . is_zero () { panic ! ("All-zero secret key") ; } let sk = SecretKey (sk) ; let pk = sk . recover_public_key () . expect ("generated public key is weak") ; KeyPair { pk , sk } } # [doc = " Check that the public key is valid for the secret key."] pub fn validate (& self) -> Result < () , Error > { self . sk . validate_public_key (& self . pk) } }
};
}
