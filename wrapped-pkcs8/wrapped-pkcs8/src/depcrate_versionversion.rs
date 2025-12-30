// Generated macro for Version (enum)
macro_rules! Depcrate_versionVersion {
() => {
// Module: crate::version
// Provides: {"Version"}
// Dependencies: {}
# [doc = " Version identifier for PKCS#8 documents."] # [doc = ""] # [doc = " (RFC 5958 designates `0` and `1` as the only valid versions for PKCS#8 documents)"] # [derive (Clone , Debug , Copy , PartialEq , Eq)] pub enum Version { # [doc = " Denotes PKCS#8 v1: no public key field."] V1 = 0 , # [doc = " Denotes PKCS#8 v2: `OneAsymmetricKey` with public key field."] V2 = 1 , }
};
}
