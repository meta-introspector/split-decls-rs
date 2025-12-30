// Generated macro for impl_43 (impl)
macro_rules! Depcrate_astimpl_43 {
() => {
// Module: crate::ast
// Provides: {"impl_43"}
// Dependencies: {}
impl AngleBracketedArg { pub fn span (& self) -> Span { match self { AngleBracketedArg :: Arg (arg) => arg . span () , AngleBracketedArg :: Constraint (constraint) => constraint . span , } } }
};
}
