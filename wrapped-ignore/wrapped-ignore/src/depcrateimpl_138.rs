// Generated macro for impl_138 (impl)
macro_rules! Depcrateimpl_138 {
() => {
// Module: crate
// Provides: {"impl_138"}
// Dependencies: {}
impl std :: error :: Error for Error { # [allow (deprecated)] fn description (& self) -> & str { match * self { Error :: Partial (_) => "partial error" , Error :: WithLineNumber { ref err , .. } => err . description () , Error :: WithPath { ref err , .. } => err . description () , Error :: WithDepth { ref err , .. } => err . description () , Error :: Loop { .. } => "file system loop found" , Error :: Io (ref err) => err . description () , Error :: Glob { ref err , .. } => err , Error :: UnrecognizedFileType (_) => "unrecognized file type" , Error :: InvalidDefinition => "invalid definition" , } } }
};
}
