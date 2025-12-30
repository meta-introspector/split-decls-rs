// Generated macro for impl_21 (impl)
macro_rules! Depcrate_errorimpl_21 {
() => {
// Module: crate::error
// Provides: {"impl_21"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { match self { Error :: Corn (err) => write ! (f , "{err}") , Error :: ReadingFile (err) => write ! (f , "{err}") , Error :: Serializing (err) => write ! (f , "The input could not be serialized into the requested output format:\n\t{err}") , } } }
};
}
