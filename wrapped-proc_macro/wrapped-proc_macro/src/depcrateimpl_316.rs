// Generated macro for impl_316 (impl)
macro_rules! Depcrateimpl_316 {
() => {
// Module: crate
// Provides: {"impl_316"}
// Dependencies: {}
# [doc = " Prints the token stream as a string that is supposed to be losslessly convertible back"] # [doc = " into the same token stream (modulo spans), except for possibly `TokenTree::Group`s"] # [doc = " with `Delimiter::None` delimiters and negative numeric literals."] # [doc = ""] # [doc = " Note: the exact form of the output is subject to change, e.g. there might"] # [doc = " be changes in the whitespace used between tokens. Therefore, you should"] # [doc = " *not* do any kind of simple substring matching on the output string (as"] # [doc = " produced by `to_string`) to implement a proc macro, because that matching"] # [doc = " might stop working if such changes happen. Instead, you should work at the"] # [doc = " `TokenTree` level, e.g. matching against `TokenTree::Ident`,"] # [doc = " `TokenTree::Punct`, or `TokenTree::Literal`."] # [stable (feature = "proc_macro_lib" , since = "1.15.0")] impl fmt :: Display for TokenStream { # [allow (clippy :: recursive_format_impl)] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . 0 { Some (ts) => write ! (f , "{}" , ts . to_string ()) , None => Ok (()) , } } }
};
}
