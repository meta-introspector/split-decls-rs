// Generated macro for BlsError (enum)
macro_rules! Depcrate_errorBlsError {
() => {
// Module: crate::error
// Provides: {"BlsError"}
// Dependencies: {}
# [derive (Error , Clone , Debug , Eq , PartialEq)] pub enum BlsError { # [error ("Field decode failed")] FieldDecode , # [error ("Empty aggregation attempted")] EmptyAggregation , # [error ("Key derivation failed")] KeyDerivation , # [error ("Point representation conversion failed")] PointConversion , # [error ("Failed to parse from string")] ParseFromString , # [error ("Failed to parse from bytes")] ParseFromBytes , # [error ("The length of inputs do not match")] InputLengthMismatch , }
};
}
