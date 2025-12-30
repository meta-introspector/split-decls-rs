// Generated macro for print_bytes (module)
macro_rules! Depcrate_baseprint_bytes {
() => {
// Module: crate::base
// Provides: {"print_bytes"}
// Dependencies: {}
# [cfg (feature = "print_bytes")] # [cfg_attr (normpath_docs_rs , doc (cfg (feature = "print_bytes")))] mod print_bytes { use print_bytes :: ByteStr ; use print_bytes :: ToBytes ; # [cfg (windows)] use print_bytes :: WideStr ; use super :: BasePath ; use super :: BasePathBuf ; impl ToBytes for BasePath { # [inline] fn to_bytes (& self) -> ByteStr < '_ > { self . 0 . to_bytes () } # [cfg (windows)] # [inline] fn to_wide (& self) -> Option < WideStr > { self . 0 . to_wide () } } impl ToBytes for BasePathBuf { # [inline] fn to_bytes (& self) -> ByteStr < '_ > { (* * self) . to_bytes () } # [cfg (windows)] # [inline] fn to_wide (& self) -> Option < WideStr > { (* * self) . to_wide () } } }
};
}
