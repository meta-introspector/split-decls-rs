// Generated macro for LitFloatType (enum)
macro_rules! Depcrate_astLitFloatType {
() => {
// Module: crate::ast
// Provides: {"LitFloatType"}
// Dependencies: {}
# [doc = " Type of the float literal based on provided suffix."] # [derive (Clone , Copy , Encodable , Decodable , Debug , Hash , Eq , PartialEq)] # [derive (HashStable_Generic)] pub enum LitFloatType { # [doc = " A float literal with a suffix (`1f32` or `1E10f32`)."] Suffixed (FloatTy) , # [doc = " A float literal without a suffix (`1.0 or 1.0E10`)."] Unsuffixed , }
};
}
