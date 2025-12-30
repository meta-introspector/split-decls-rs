// Generated macro for impl_361 (impl)
macro_rules! Depcrateimpl_361 {
() => {
// Module: crate
// Provides: {"impl_361"}
// Dependencies: {}
# [doc = " Prints token tree in a form convenient for debugging."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] impl fmt :: Debug for TokenTree { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { TokenTree :: Group (ref tt) => tt . fmt (f) , TokenTree :: Ident (ref tt) => tt . fmt (f) , TokenTree :: Punct (ref tt) => tt . fmt (f) , TokenTree :: Literal (ref tt) => tt . fmt (f) , } } }
};
}
