// Generated macro for impl_298 (impl)
macro_rules! Depcrate_hazardous_hash_blake2_blake2bimpl_298 {
() => {
// Module: crate::hazardous::hash::blake2::blake2b
// Provides: {"impl_298"}
// Dependencies: {}
impl Hasher { # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Return a digest selected by the given Blake2b variant."] pub fn digest (& self , data : & [u8]) -> Result < Digest , UnknownCryptoError > { let size : usize = match * self { Hasher :: Blake2b256 => 32 , Hasher :: Blake2b384 => 48 , Hasher :: Blake2b512 => 64 , } ; let mut state = Blake2b :: new (size) ? ; state . update (data) ? ; state . finalize () } # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Return a `Blake2b` state selected by the given Blake2b variant."] pub fn init (& self) -> Result < Blake2b , UnknownCryptoError > { match * self { Hasher :: Blake2b256 => Blake2b :: new (32) , Hasher :: Blake2b384 => Blake2b :: new (48) , Hasher :: Blake2b512 => Blake2b :: new (64) , } } }
};
}
