// Generated macro for cksum (function)
macro_rules! Depcrate_registrycksum {
() => {
// Module: crate::registry
// Provides: {"cksum"}
// Dependencies: {}
# [doc = " Generate a checksum"] pub fn cksum (s : & [u8]) -> String { Sha256 :: new () . update (s) . finish_hex () }
};
}
