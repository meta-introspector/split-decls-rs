// Generated macro for impl_321 (impl)
macro_rules! Depcrateimpl_321 {
() => {
// Module: crate
// Provides: {"impl_321"}
// Dependencies: {}
# [doc = " Creates a token stream containing a single token tree."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] impl From < TokenTree > for TokenStream { fn from (tree : TokenTree) -> TokenStream { TokenStream (Some (bridge :: client :: TokenStream :: from_token_tree (tree_to_bridge_tree (tree)))) } }
};
}
