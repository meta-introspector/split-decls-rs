// Generated macro for minimal_quote (macro)
macro_rules! Depcrate_quoteminimal_quote {
() => {
// Module: crate::quote
// Provides: {"minimal_quote"}
// Dependencies: {}
# [doc = " Simpler version of the real `quote!` macro, implemented solely"] # [doc = " through `macro_rules`, for bootstrapping the real implementation"] # [doc = " (see the `quote` function), which does not have access to the"] # [doc = " real `quote!` macro due to the `proc_macro` crate not being"] # [doc = " able to depend on itself."] # [doc = ""] # [doc = " Note: supported tokens are a subset of the real `quote!`, but"] # [doc = " unquoting is different: instead of `$x`, this uses `(@ expr)`."] macro_rules ! minimal_quote { ($ ($ t : tt) *) => { { # [allow (unused_mut)] let mut ts = TokenStream :: new () ; $ (ToTokens :: to_tokens (& minimal_quote_ts ! ($ t) , & mut ts) ;) * ts } } ; }
};
}
