// Generated macro for impl_1424 (impl)
macro_rules! Depcrate_stringimpl_1424 {
() => {
// Module: crate::string
// Provides: {"impl_1424"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: RegexSyntax (err) => write ! (f , "{}" , err) , Error :: UnsupportedRegex (message) => write ! (f , "{}" , message) , } } }
};
}
