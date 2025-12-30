// Generated macro for impl_123 (impl)
macro_rules! Depcrate_errorimpl_123 {
() => {
// Module: crate::error
// Provides: {"impl_123"}
// Dependencies: {}
impl :: std :: error :: Error for Error { fn description (& self) -> & str { match * self { Error :: Syntax (ref err) => err . description () , Error :: CompiledTooBig (_) => "compiled program too big" , Error :: InvalidSet => { "sets must contain 2 or more regular expressions" } Error :: __Nonexhaustive => unreachable ! () , } } fn cause (& self) -> Option < & :: std :: error :: Error > { match * self { Error :: Syntax (ref err) => Some (err) , _ => None , } } }
};
}
