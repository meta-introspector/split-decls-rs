// Generated macro for impl_66 (impl)
macro_rules! Depcrate_errorsimpl_66 {
() => {
// Module: crate::errors
// Provides: {"impl_66"}
// Dependencies: {}
impl std :: fmt :: Display for FluentError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Overriding { kind , id } => { write ! (f , "Attempt to override an existing {}: \"{}\"." , kind , id) } Self :: ParserError (err) => write ! (f , "Parser error: {}" , err) , Self :: ResolverError (err) => write ! (f , "Resolver error: {}" , err) , } } }
};
}
