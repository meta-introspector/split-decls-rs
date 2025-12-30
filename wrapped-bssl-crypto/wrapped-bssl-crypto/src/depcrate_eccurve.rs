// Generated macro for Curve (trait)
macro_rules! Depcrate_ecCurve {
() => {
// Module: crate::ec
// Provides: {"Curve"}
// Dependencies: {}
# [doc = " An elliptic curve."] pub trait Curve : Debug { # [doc (hidden)] fn group (_ : sealed :: Sealed) -> Group ; # [doc = " Hash `data` using a hash function suitable for the curve. (I.e."] # [doc = " SHA-256 for P-256 and SHA-384 for P-384.)"] # [doc (hidden)] fn hash (data : & [u8]) -> Vec < u8 > ; }
};
}
