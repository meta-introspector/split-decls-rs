// Generated macro for impl_372 (impl)
macro_rules! Depcrateimpl_372 {
() => {
// Module: crate
// Provides: {"impl_372"}
// Dependencies: {}
# [doc = " Prints the group as a string that should be losslessly convertible back"] # [doc = " into the same group (modulo spans), except for possibly `TokenTree::Group`s"] # [doc = " with `Delimiter::None` delimiters."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] impl fmt :: Display for Group { # [allow (clippy :: recursive_format_impl)] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , TokenStream :: from (TokenTree :: from (self . clone ()))) } }
};
}
