// Generated macro for impl_175 (impl)
macro_rules! Depcrate_contentimpl_175 {
() => {
// Module: crate::content
// Provides: {"impl_175"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: FailedParsingYaml (p) => { f . write_str (format ! ("Failed parsing the YAML from {:?}" , p . display ()) . as_str ()) } Error :: UnexpectedDataType => { f . write_str ("The present data type wasn't what was expected") } Error :: MissingField => f . write_str ("A required field was missing") , Error :: FileIo (e , p) => { f . write_str (format ! ("File error for {:?}: {}" , p . display () , e) . as_str ()) } } } }
};
}
