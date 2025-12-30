// Generated macro for impl_23 (impl)
macro_rules! Depcrate_astimpl_23 {
() => {
// Module: crate::ast
// Provides: {"impl_23"}
// Dependencies: {}
impl Display for GraphFromFileError < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { match self { Self :: FileError (e) => write ! (f , "{}" , e) , Self :: PestParseError (e) => write ! (f , "{}" , e) , Self :: ParseError (e) => write ! (f , "{}" , e) , } } }
};
}
