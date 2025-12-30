// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { Error :: NonStringLiteral => f . write_str ("expected string literal") , Error :: UuidParse (_ , ref e) => write ! (f , "{}" , e) , } } }
};
}
