// Generated macro for macro_2671 (macro)
macro_rules! Depcrate_pg_expression_functionsmacro_2671 {
() => {
// Module: crate::pg::expression::functions
// Provides: {"macro_2671"}
// Dependencies: {}
define_sql_function ! { # [doc = " Computes the host mask for the address's network."] # [cfg (feature = "postgres_backend")] fn hostmask < T : InetOrCidr + SingleValue > (addr : T) -> Inet ; }
};
}
