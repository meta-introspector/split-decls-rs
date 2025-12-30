// Generated macro for MfType (enum)
macro_rules! Depcrate_lz_lz_encoderMfType {
() => {
// Module: crate::lz::lz_encoder
// Provides: {"MfType"}
// Dependencies: {}
# [doc = " Match finders to use when encoding."] # [derive (Default , Debug , Clone , Copy , PartialEq , Eq)] pub enum MfType { # [doc = " Hash chain for 4 bytes entries (lower quality but faster)."] # [default] Hc4 , # [doc = " Binary tree for 4 byte entries (higher quality but slower)."] Bt4 , }
};
}
