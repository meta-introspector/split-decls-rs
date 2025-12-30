// Generated macro for impl_725 (impl)
macro_rules! Depcrate_svhimpl_725 {
() => {
// Module: crate::svh
// Provides: {"impl_725"}
// Dependencies: {}
impl Svh { # [doc = " Creates a new `Svh` given the hash. If you actually want to"] # [doc = " compute the SVH from some HIR, you want the `calculate_svh`"] # [doc = " function found in `rustc_incremental`."] pub fn new (hash : Fingerprint) -> Svh { Svh { hash } } pub fn as_u128 (self) -> u128 { self . hash . as_u128 () } pub fn to_hex (self) -> String { format ! ("{:032x}" , self . hash . as_u128 ()) } }
};
}
