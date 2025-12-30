// Generated macro for impl_7 (impl)
macro_rules! Depcrate_errorimpl_7 {
() => {
// Module: crate::error
// Provides: {"impl_7"}
// Dependencies: {}
impl < I > ContextError < I > for VerboseError < I > { fn add_context (input : I , ctx : & 'static str , mut other : Self) -> Self { other . errors . push ((input , VerboseErrorKind :: Context (ctx))) ; other } }
};
}
