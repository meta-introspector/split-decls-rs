// Generated macro for ScalarBits (type)
macro_rules! DepcrateScalarBits {
() => {
// Module: crate
// Provides: {"ScalarBits"}
// Dependencies: {}
# [doc = " Bit representation of a NIST P-384 scalar field element."] # [cfg (feature = "bits")] pub type ScalarBits = elliptic_curve :: scalar :: ScalarBits < NistP384 > ;
};
}
