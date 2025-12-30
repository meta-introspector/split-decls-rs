// Generated macro for impl_801 (impl)
macro_rules! Depcrate_oidimpl_801 {
() => {
// Module: crate::oid
// Provides: {"impl_801"}
// Dependencies: {}
impl str :: FromStr for Oid { type Err = Error ; # [doc = " Parse a hex-formatted object id into an Oid structure."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns an error if the string is empty, is longer than 40 hex"] # [doc = " characters, or contains any non-hex characters."] fn from_str (s : & str) -> Result < Oid , Error > { Oid :: from_str (s) } }
};
}
