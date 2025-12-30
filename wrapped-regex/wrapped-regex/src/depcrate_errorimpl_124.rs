// Generated macro for impl_124 (impl)
macro_rules! Depcrate_errorimpl_124 {
() => {
// Module: crate::error
// Provides: {"impl_124"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { Error :: Syntax (ref err) => err . fmt (f) , Error :: CompiledTooBig (limit) => { write ! (f , "Compiled regex exceeds size limit of {} bytes." , limit) } Error :: InvalidSet => { write ! (f , "Sets must contain 2 or more regular expressions.") } Error :: __Nonexhaustive => unreachable ! () , } } }
};
}
