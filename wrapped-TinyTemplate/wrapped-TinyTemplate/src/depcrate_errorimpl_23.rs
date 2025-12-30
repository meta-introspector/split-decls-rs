// Generated macro for impl_23 (impl)
macro_rules! Depcrate_errorimpl_23 {
() => {
// Module: crate::error
// Provides: {"impl_23"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Error :: ParseError { msg , line , column } => write ! (f , "Failed to parse the template (line {}, column {}). Reason: {}" , line , column , msg) , Error :: RenderError { msg , line , column } => { write ! (f , "Encountered rendering error on line {}, column {}. Reason: {}" , line , column , msg) } Error :: SerdeError { err } => { write ! (f , "Unexpected serde error while converting the context to a serde_json::Value. Error: {}" , err) } Error :: GenericError { msg } => { write ! (f , "{}" , msg) } Error :: StdFormatError { err } => { write ! (f , "Unexpected formatting error: {}" , err) } Error :: CalledTemplateError { name , err , line , column , } => { write ! (f , "Call to sub-template \"{}\" on line {}, column {} failed. Reason: {}" , name , line , column , err) } Error :: CalledFormatterError { name , err , line , column , } => { write ! (f , "Call to value formatter \"{}\" on line {}, column {} failed. Reason: {}" , name , line , column , err) } Error :: __NonExhaustive => unreachable ! () , } } }
};
}
