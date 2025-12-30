// Generated macro for other_331 (other)
macro_rules! Depcrateother_331 {
() => {
// Module: crate
// Provides: {"other_331"}
// Dependencies: {}
# [doc = " `quote!(..)` accepts arbitrary tokens and expands into a `TokenStream` describing the input."] # [doc = " For example, `quote!(a + b)` will produce an expression, that, when evaluated, constructs"] # [doc = " the `TokenStream` `[Ident(\"a\"), Punct('+', Alone), Ident(\"b\")]`."] # [doc = ""] # [doc = " Unquoting is done with `$`, and works by taking the single next ident as the unquoted term."] # [doc = " To quote `$` itself, use `$$`."] # [unstable (feature = "proc_macro_quote" , issue = "54722")] # [allow_internal_unstable (proc_macro_def_site , proc_macro_internals , proc_macro_totokens)] # [rustc_builtin_macro] pub macro quote ($ ($ t : tt) *) { }
};
}
