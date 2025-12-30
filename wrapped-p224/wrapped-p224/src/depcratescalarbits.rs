// Generated macro for ScalarBits (type)
macro_rules! DepcrateScalarBits {
() => {
// Module: crate
// Provides: {"ScalarBits"}
// Dependencies: {}
# [doc = " Bit representation of a NIST P-224 scalar field element."] # [cfg (feature = "bits")] pub type ScalarBits = elliptic_curve :: scalar :: ScalarBits < NistP224 > ;
};
}
