// Generated macro for SignatureSize (type)
macro_rules! DepcrateSignatureSize {
() => {
// Module: crate
// Provides: {"SignatureSize"}
// Dependencies: {}
# [doc = " Size of a fixed sized signature for the given elliptic curve."] pub type SignatureSize < C > = < FieldBytesSize < C > as Add > :: Output ;
};
}
