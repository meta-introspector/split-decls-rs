// Generated macro for EscapeFn (type)
macro_rules! Depcrate_registryEscapeFn {
() => {
// Module: crate::registry
// Provides: {"EscapeFn"}
// Dependencies: {}
# [doc = " This type represents an *escape fn*, that is a function whose purpose it is"] # [doc = " to escape potentially problematic characters in a string."] # [doc = ""] # [doc = " An *escape fn* is represented as a `Box` to avoid unnecessary type"] # [doc = " parameters (and because traits cannot be aliased using `type`)."] pub type EscapeFn = Arc < dyn Fn (& str) -> String + Send + Sync > ;
};
}
