// Generated macro for impl_486 (impl)
macro_rules! Depcrate_hazardous_ecc_x25519impl_486 {
() => {
// Module: crate::hazardous::ecc::x25519
// Provides: {"impl_486"}
// Dependencies: {}
impl PrivateKey { # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Construct from a given byte slice."] pub fn from_slice (slice : & [u8]) -> Result < Self , UnknownCryptoError > { Ok (Self { scalar : Scalar :: from_slice (slice) ? , }) } # [inline] # [doc = " Return the length of the object."] pub fn len (& self) -> usize { PRIVATE_KEY_SIZE } # [inline] # [doc = " Return `true` if this object does not hold any data, `false` otherwise."] # [doc = ""] # [doc = " __NOTE__: This method should always return `false`, since there shouldn't be a way"] # [doc = " to create an empty instance of this object."] pub fn is_empty (& self) -> bool { PRIVATE_KEY_SIZE == 0 } # [inline] # [doc = " Return the object as byte slice. __**Warning**__: Should not be used unless strictly"] # [doc = " needed. This __**breaks protections**__ that the type implements."] pub fn unprotected_as_bytes (& self) -> & [u8] { self . scalar . 0 . as_ref () } # [cfg (feature = "safe_api")] # [cfg_attr (docsrs , doc (cfg (feature = "safe_api")))] # [doc = " Randomly generate using a CSPRNG. Not available in `no_std` context."] pub fn generate () -> PrivateKey { let mut value = [0u8 ; PRIVATE_KEY_SIZE] ; crate :: util :: secure_rand_bytes (& mut value) . unwrap () ; Self { scalar : Scalar :: from_slice (& value) . unwrap () , } } }
};
}
