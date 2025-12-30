// Generated macro for impl_24 (impl)
macro_rules! Depcrate_errorimpl_24 {
() => {
// Module: crate::error
// Provides: {"impl_24"}
// Dependencies: {}
impl StdError for Error { fn description (& self) -> & str { match self { Error :: ParseError { .. } => "ParseError" , Error :: RenderError { .. } => "RenderError" , Error :: SerdeError { .. } => "SerdeError" , Error :: GenericError { msg } => & msg , Error :: StdFormatError { .. } => "StdFormatError" , Error :: CalledTemplateError { .. } => "CalledTemplateError" , Error :: CalledFormatterError { .. } => "CalledFormatterError" , Error :: __NonExhaustive => unreachable ! () , } } }
};
}
