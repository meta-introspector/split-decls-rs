// Generated macro for MacroInput (struct)
macro_rules! Depcrate_formatMacroInput {
() => {
// Module: crate::format
// Provides: {"MacroInput"}
// Dependencies: {}
# [derive (Debug)] struct MacroInput { fmtstr : Box < Expr > , args : FormatArguments , # [doc = " Whether the first argument was a string literal or a result from eager macro expansion."] # [doc = " If it's not a string literal, we disallow implicit argument capturing."] # [doc = ""] # [doc = " This does not correspond to whether we can treat spans to the literal normally, as the whole"] # [doc = " invocation might be the result of another macro expansion, in which case this flag may still be true."] # [doc = ""] # [doc = " See [RFC 2795] for more information."] # [doc = ""] # [doc = " [RFC 2795]: https://rust-lang.github.io/rfcs/2795-format-args-implicit-identifiers.html#macro-hygiene"] is_direct_literal : bool , }
};
}
