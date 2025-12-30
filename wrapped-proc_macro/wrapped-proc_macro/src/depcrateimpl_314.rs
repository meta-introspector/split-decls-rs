// Generated macro for impl_314 (impl)
macro_rules! Depcrateimpl_314 {
() => {
// Module: crate
// Provides: {"impl_314"}
// Dependencies: {}
impl TokenStream { # [doc = " Returns an empty `TokenStream` containing no token trees."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] pub fn new () -> TokenStream { TokenStream (None) } # [doc = " Checks if this `TokenStream` is empty."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] pub fn is_empty (& self) -> bool { self . 0 . as_ref () . map (| h | h . is_empty ()) . unwrap_or (true) } # [doc = " Parses this `TokenStream` as an expression and attempts to expand any"] # [doc = " macros within it. Returns the expanded `TokenStream`."] # [doc = ""] # [doc = " Currently only expressions expanding to literals will succeed, although"] # [doc = " this may be relaxed in the future."] # [doc = ""] # [doc = " NOTE: In error conditions, `expand_expr` may leave macros unexpanded,"] # [doc = " report an error, failing compilation, and/or return an `Err(..)`. The"] # [doc = " specific behavior for any error condition, and what conditions are"] # [doc = " considered errors, is unspecified and may change in the future."] # [unstable (feature = "proc_macro_expand" , issue = "90765")] pub fn expand_expr (& self) -> Result < TokenStream , ExpandError > { let stream = self . 0 . as_ref () . ok_or (ExpandError) ? ; match bridge :: client :: TokenStream :: expand_expr (stream) { Ok (stream) => Ok (TokenStream (Some (stream))) , Err (_) => Err (ExpandError) , } } }
};
}
