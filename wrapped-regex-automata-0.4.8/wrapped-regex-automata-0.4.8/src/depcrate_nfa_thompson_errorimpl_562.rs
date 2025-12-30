// Generated macro for impl_562 (impl)
macro_rules! Depcrate_nfa_thompson_errorimpl_562 {
() => {
// Module: crate::nfa::thompson::error
// Provides: {"impl_562"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for BuildError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self . kind () { # [cfg (feature = "syntax")] BuildErrorKind :: Syntax (ref err) => Some (err) , BuildErrorKind :: Captures (ref err) => Some (err) , _ => None , } } }
};
}
