// Generated macro for impl_82 (impl)
macro_rules! Depcrate_ed25519impl_82 {
() => {
// Module: crate::ed25519
// Provides: {"impl_82"}
// Dependencies: {}
impl SecretKey { # [doc = " Number of bytes in a secret key."] pub const BYTES : usize = 32 + PublicKey :: BYTES ; # [doc = " Creates a secret key from raw bytes."] pub fn new (sk : [u8 ; SecretKey :: BYTES]) -> Self { SecretKey (sk) } # [doc = " Creates a secret key from a slice."] pub fn from_slice (sk : & [u8]) -> Result < Self , Error > { let mut sk_ = [0u8 ; SecretKey :: BYTES] ; if sk . len () != sk_ . len () { return Err (Error :: InvalidSecretKey) ; } sk_ . copy_from_slice (sk) ; Ok (SecretKey :: new (sk_)) } # [doc = " Returns the public counterpart of a secret key."] pub fn public_key (& self) -> PublicKey { let mut pk = [0u8 ; PublicKey :: BYTES] ; pk . copy_from_slice (& self [Seed :: BYTES ..]) ; PublicKey (pk) } # [doc = " Returns the seed of a secret key."] pub fn seed (& self) -> Seed { Seed :: from_slice (& self [0 .. Seed :: BYTES]) . unwrap () } # [doc = " Returns `Ok(())` if the given public key is the public counterpart of"] # [doc = " this secret key."] # [doc = " Returns `Err(Error::InvalidPublicKey)` otherwise."] # [doc = " The public key is recomputed (not just copied) from the secret key,"] # [doc = " so this will detect corruption of the secret key."] pub fn validate_public_key (& self , pk : & PublicKey) -> Result < () , Error > { let kp = KeyPair :: from_seed (self . seed ()) ; if kp . pk != * pk { return Err (Error :: InvalidPublicKey) ; } Ok (()) } }
};
}
