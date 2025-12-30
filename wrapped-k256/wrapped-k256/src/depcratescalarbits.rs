// Generated macro for ScalarBits (type)
macro_rules! DepcrateScalarBits {
() => {
// Module: crate
// Provides: {"ScalarBits"}
// Dependencies: {}
# [doc = " Bit representation of a secp256k1 (K-256) scalar field element."] # [cfg (feature = "bits")] pub type ScalarBits = elliptic_curve :: scalar :: ScalarBits < Secp256k1 > ;
};
}
