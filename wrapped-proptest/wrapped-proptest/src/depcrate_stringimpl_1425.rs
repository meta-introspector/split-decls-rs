// Generated macro for impl_1425 (impl)
macro_rules! Depcrate_stringimpl_1425 {
() => {
// Module: crate::string
// Provides: {"impl_1425"}
// Dependencies: {}
impl std :: error :: Error for Error { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { Error :: RegexSyntax (err) => Some (err) , Error :: UnsupportedRegex (_) => None , } } }
};
}
