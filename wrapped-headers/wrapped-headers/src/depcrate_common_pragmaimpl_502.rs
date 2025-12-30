// Generated macro for impl_502 (impl)
macro_rules! Depcrate_common_pragmaimpl_502 {
() => {
// Module: crate::common::pragma
// Provides: {"impl_502"}
// Dependencies: {}
impl Pragma { # [doc = " Construct the literal `no-cache` Pragma header."] pub fn no_cache () -> Pragma { Pragma (HeaderValue :: from_static ("no-cache")) } # [doc = " Return whether this pragma is `no-cache`."] pub fn is_no_cache (& self) -> bool { self . 0 == "no-cache" } }
};
}
