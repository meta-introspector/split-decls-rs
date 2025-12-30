// Generated macro for impl_420 (impl)
macro_rules! Depcrate_options_parserimpl_420 {
() => {
// Module: crate::options::parser
// Provides: {"impl_420"}
// Dependencies: {}
impl fmt :: Display for ParseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: NeedsValue { flag , values : None } => write ! (f , "Flag {flag} needs a value") , Self :: NeedsValue { flag , values : Some (cs) , } => write ! (f , "Flag {flag} needs a value ({})" , Choices (cs)) , Self :: ForbiddenValue { flag } => write ! (f , "Flag {flag} cannot take a value") , Self :: UnknownShortArgument { attempt } => { write ! (f , "Unknown argument -{}" , * attempt as char) } Self :: UnknownArgument { attempt } => { write ! (f , "Unknown argument --{}" , attempt . to_string_lossy ()) } } } }
};
}
