// Generated macro for Extensions (struct)
macro_rules! Depcrate_extensionsExtensions {
() => {
// Module: crate::extensions
// Provides: {"Extensions"}
// Dependencies: {}
# [doc = " An invariant-enforcing wrapper for `RawExtensions`."] # [doc = ""] # [doc = " In particular, an `Extensions` cannot be constructed from a `RawExtensions`"] # [doc = " that contains duplicated extensions (by OID)."] pub struct Extensions < 'a > (Option < RawExtensions < 'a > >) ;
};
}
