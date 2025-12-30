// Generated macro for impl_22 (impl)
macro_rules! Depcrate_errorimpl_22 {
() => {
// Module: crate::error
// Provides: {"impl_22"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for Error { # [allow (deprecated)] fn description (& self) -> & str { match * self { Error :: Syntax (ref err) => err , Error :: CompiledTooBig (_) => "compiled program too big" , } } }
};
}
