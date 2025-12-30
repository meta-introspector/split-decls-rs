// Generated macro for impl_139 (impl)
macro_rules! Depcrateimpl_139 {
() => {
// Module: crate
// Provides: {"impl_139"}
// Dependencies: {}
impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match * self { Error :: Partial (ref errs) => { let msgs : Vec < String > = errs . iter () . map (| err | err . to_string ()) . collect () ; write ! (f , "{}" , msgs . join ("\n")) } Error :: WithLineNumber { line , ref err } => { write ! (f , "line {}: {}" , line , err) } Error :: WithPath { ref path , ref err } => { write ! (f , "{}: {}" , path . display () , err) } Error :: WithDepth { ref err , .. } => err . fmt (f) , Error :: Loop { ref ancestor , ref child } => write ! (f , "File system loop found: \
                           {} points to an ancestor {}" , child . display () , ancestor . display ()) , Error :: Io (ref err) => err . fmt (f) , Error :: Glob { glob : None , ref err } => write ! (f , "{}" , err) , Error :: Glob { glob : Some (ref glob) , ref err } => { write ! (f , "error parsing glob '{}': {}" , glob , err) } Error :: UnrecognizedFileType (ref ty) => { write ! (f , "unrecognized file type: {}" , ty) } Error :: InvalidDefinition => write ! (f , "invalid definition (format is type:glob, e.g., \
                           html:*.html)") , } } }
};
}
