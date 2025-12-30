// Generated macro for ci (function)
macro_rules! Depcrateci {
() => {
// Module: crate
// Provides: {"ci"}
// Dependencies: {}
# [doc = " True if `CI` is set and nonempty."] pub const fn ci () -> bool { match option_env ! ("CI") { Some (s) if s . is_empty () => false , None => false , Some (_) => true , } }
};
}
