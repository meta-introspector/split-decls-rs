// Generated macro for impl_302 (impl)
macro_rules! Depcrate_error_key_rejectedimpl_302 {
() => {
// Module: crate::error::key_rejected
// Provides: {"impl_302"}
// Dependencies: {}
impl KeyRejected { pub (crate) fn inconsistent_components () -> Self { Self ("InconsistentComponents") } pub (crate) fn invalid_component () -> Self { Self ("InvalidComponent") } # [inline] pub (crate) fn invalid_encoding () -> Self { Self ("InvalidEncoding") } pub (crate) fn rng_failed () -> Self { Self ("RNG failed") } pub (crate) fn public_key_is_missing () -> Self { Self ("PublicKeyIsMissing") } # [cfg (feature = "alloc")] pub (crate) const fn too_small () -> Self { Self ("TooSmall") } # [cfg (feature = "alloc")] pub (crate) fn too_large () -> Self { Self ("TooLarge") } pub (crate) fn version_not_supported () -> Self { Self ("VersionNotSupported") } pub (crate) fn wrong_algorithm () -> Self { Self ("WrongAlgorithm") } # [cfg (feature = "alloc")] pub (crate) fn private_modulus_len_not_multiple_of_512_bits () -> Self { Self ("PrivateModulusLenNotMultipleOf512Bits") } pub (crate) fn unexpected_error () -> Self { Self ("UnexpectedError") } # [cfg (test)] pub (crate) fn eq (& self , other : Self) -> bool { self . 0 == other . 0 } }
};
}
