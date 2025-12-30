// Generated macro for impl_43 (impl)
macro_rules! Depcrate_errorimpl_43 {
() => {
// Module: crate::error
// Provides: {"impl_43"}
// Dependencies: {}
impl < R : RuleType > fmt :: Display for ErrorVariant < R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ErrorVariant :: ParsingError { .. } => write ! (f , "parsing error: {}" , self . message ()) , ErrorVariant :: CustomError { .. } => write ! (f , "{}" , self . message ()) , } } }
};
}
