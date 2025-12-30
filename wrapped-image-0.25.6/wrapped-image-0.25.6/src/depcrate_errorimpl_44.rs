// Generated macro for impl_44 (impl)
macro_rules! Depcrate_errorimpl_44 {
() => {
// Module: crate::error
// Provides: {"impl_44"}
// Dependencies: {}
impl fmt :: Display for ParameterError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { match & self . kind { ParameterErrorKind :: DimensionMismatch => write ! (fmt , "The Image's dimensions are either too \
                 small or too large") , ParameterErrorKind :: FailedAlready => write ! (fmt , "The end the image stream has been reached due to a previous error") , ParameterErrorKind :: Generic (message) => { write ! (fmt , "The parameter is malformed: {message}" ,) } ParameterErrorKind :: NoMoreData => write ! (fmt , "The end of the image has been reached" ,) , } ? ; if let Some (underlying) = & self . underlying { write ! (fmt , "\n{underlying}") ? ; } Ok (()) } }
};
}
