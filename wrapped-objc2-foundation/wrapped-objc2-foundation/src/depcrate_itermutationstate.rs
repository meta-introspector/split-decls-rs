// Generated macro for MutationState (type)
macro_rules! Depcrate_iterMutationState {
() => {
// Module: crate::iter
// Provides: {"MutationState"}
// Dependencies: {}
# [doc = " Track mutation mistakes when debug assertions are enabled (they should"] # [doc = " be made impossible by Rust at compile-time, so hence we don't need to"] # [doc = " track them in release mode)."] # [doc = ""] # [doc = " This is set to `None` initially, but later loaded to `Some(_)` after"] # [doc = " the first enumeration."] type MutationState = Option < c_ulong > ;
};
}
