// Generated macro for add_child (macro)
macro_rules! Depcrate_erroradd_child {
() => {
// Module: crate::error
// Provides: {"add_child"}
// Dependencies: {}
# [cfg (feature = "diagnostics")] macro_rules ! add_child { ($ unspanned : ident , $ spanned : ident , $ level : ident) => { # [doc = concat ! ("Add a child " , stringify ! ($ unspanned) , " message to this error.")] # [doc = "# Example"] # [doc = "```rust"] # [doc = "# use darling_core::Error;"] # [doc = concat ! (r#"Error::custom("Example")."# , stringify ! ($ unspanned) , r#"("message content");"#)] # [doc = "```"] pub fn $ unspanned < T : fmt :: Display > (mut self , message : T) -> Self { self . children . push (child :: ChildDiagnostic :: new (child :: Level ::$ level , None , message . to_string () ,)) ; self } # [doc = concat ! ("Add a child " , stringify ! ($ unspanned) , " message to this error with its own span.")] # [doc = "# Example"] # [doc = "```rust"] # [doc = "# use darling_core::Error;"] # [doc = "# let item_to_span = proc_macro2::Span::call_site();"] # [doc = concat ! (r#"Error::custom("Example")."# , stringify ! ($ spanned) , r#"(&item_to_span, "message content");"#)] # [doc = "```"] pub fn $ spanned < S : Spanned , T : fmt :: Display > (mut self , span : & S , message : T) -> Self { self . children . push (child :: ChildDiagnostic :: new (child :: Level ::$ level , Some (span . span ()) , message . to_string () ,)) ; self } } ; }
};
}
