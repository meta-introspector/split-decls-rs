// Generated macro for UncompressedPublicKey (struct)
macro_rules! Depcrate_version3UncompressedPublicKey {
() => {
// Module: crate::version3
// Provides: {"UncompressedPublicKey"}
// Dependencies: {}
# [doc = " This struct represents a uncompressed public key for P384, encoded in big-endian using:"] # [doc = " Octet-String-to-Elliptic-Curve-Point algorithm in SEC 1: Elliptic Curve Cryptography, Version 2.0."] # [doc = ""] # [doc = " Format: `[0x04 || x || y]`"] # [doc = ""] # [doc = " This is provided to be able to convert uncompressed keys to compressed ones, as compressed is"] # [doc = " required by PASETO and what an [`AsymmetricPublicKey<V3>`] represents."] pub struct UncompressedPublicKey (PublicKey) ;
};
}
