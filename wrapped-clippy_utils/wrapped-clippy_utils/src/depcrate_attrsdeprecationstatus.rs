// Generated macro for DeprecationStatus (enum)
macro_rules! Depcrate_attrsDeprecationStatus {
() => {
// Module: crate::attrs
// Provides: {"DeprecationStatus"}
// Dependencies: {}
# [doc = " Deprecation status of attributes known by Clippy."] pub enum DeprecationStatus { # [doc = " Attribute is deprecated"] Deprecated , # [doc = " Attribute is deprecated and was replaced by the named attribute"] Replaced (& 'static str) , None , }
};
}
