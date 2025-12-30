// Generated macro for Version (enum)
macro_rules! Depcrate_pfxVersion {
() => {
// Module: crate::pfx
// Provides: {"Version"}
// Dependencies: {}
# [doc = " just the version v3"] # [derive (Clone , Copy , Debug , Enumerated , Eq , PartialEq , PartialOrd , Ord)] # [asn1 (type = "INTEGER")] # [repr (u8)] pub enum Version { # [doc = " syntax version 3"] V3 = 3 , }
};
}
