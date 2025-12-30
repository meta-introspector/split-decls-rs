// Generated macro for Context (struct)
macro_rules! Depcrate_hmacContext {
() => {
// Module: crate::hmac
// Provides: {"Context"}
// Dependencies: {}
# [doc = " A context for multi-step (Init-Update-Finish) HMAC signing."] # [doc = ""] # [doc = " Use `sign` for single-step HMAC signing."] # [derive (Clone)] pub struct Context { inner : digest :: Context , outer : digest :: BlockContext , }
};
}
