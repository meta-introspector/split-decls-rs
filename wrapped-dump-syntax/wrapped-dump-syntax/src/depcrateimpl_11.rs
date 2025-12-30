// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Error :: IncorrectUsage => write ! (f , "Usage: dump-syntax path/to/filename.rs") , Error :: ReadFile (error) => write ! (f , "Unable to read file: {}" , error) , Error :: ParseFile { error , filepath , source_code , } => render_location (f , error , filepath , source_code) , } } }
};
}
