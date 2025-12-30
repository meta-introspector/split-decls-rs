// Generated macro for strip_brackets (function)
macro_rules! Depcrate_registrystrip_brackets {
() => {
// Module: crate::registry
// Provides: {"strip_brackets"}
// Dependencies: {}
fn strip_brackets (type_name : & str) -> Option < & str > { type_name . strip_prefix ('[') . map (| rest | & rest [.. rest . len () - 1]) }
};
}
