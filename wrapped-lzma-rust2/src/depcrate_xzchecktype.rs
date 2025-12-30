// Generated macro for CheckType (enum)
macro_rules! Depcrate_xzCheckType {
() => {
// Module: crate::xz
// Provides: {"CheckType"}
// Dependencies: {}
# [doc = " Supported checksum types in XZ format."] # [derive (Default , Debug , Clone , Copy , PartialEq , Eq)] pub enum CheckType { # [doc = " No checksum"] None = 0x00 , # [doc = " CRC32"] Crc32 = 0x01 , # [doc = " CRC64"] # [default] Crc64 = 0x04 , # [doc = " SHA-256"] Sha256 = 0x0A , }
};
}
