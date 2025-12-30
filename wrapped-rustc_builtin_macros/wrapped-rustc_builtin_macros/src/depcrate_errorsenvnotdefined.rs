// Generated macro for EnvNotDefined (enum)
macro_rules! Depcrate_errorsEnvNotDefined {
() => {
// Module: crate::errors
// Provides: {"EnvNotDefined"}
// Dependencies: {}
# [derive (Diagnostic)] pub (crate) enum EnvNotDefined < 'a > { # [diag (builtin_macros_env_not_defined)] # [help (builtin_macros_cargo)] CargoEnvVar { # [primary_span] span : Span , var : Symbol , var_expr : & 'a rustc_ast :: Expr , } , # [diag (builtin_macros_env_not_defined)] # [help (builtin_macros_custom)] CustomEnvVar { # [primary_span] span : Span , var : Symbol , var_expr : & 'a rustc_ast :: Expr , } , }
};
}
