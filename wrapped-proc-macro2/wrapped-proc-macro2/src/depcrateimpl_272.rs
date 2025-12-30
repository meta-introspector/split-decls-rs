// Generated macro for impl_272 (impl)
macro_rules! Depcrateimpl_272 {
() => {
// Module: crate
// Provides: {"impl_272"}
// Dependencies: {}
# [doc = " Prints the token tree as a string that is supposed to be losslessly"] # [doc = " convertible back into the same token tree (modulo spans), except for"] # [doc = " possibly `TokenTree::Group`s with `Delimiter::None` delimiters and negative"] # [doc = " numeric literals."] impl Display for TokenTree { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { TokenTree :: Group (t) => Display :: fmt (t , f) , TokenTree :: Ident (t) => Display :: fmt (t , f) , TokenTree :: Punct (t) => Display :: fmt (t , f) , TokenTree :: Literal (t) => Display :: fmt (t , f) , } } }
};
}
